#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{State, Manager};
use tonic::transport::Channel;
use validblock::anchor_service_client::AnchorServiceClient;
use validblock::verify_service_client::VerifyServiceClient;
use validblock::anchor_service_server::AnchorServiceServer;
use validblock::verify_service_server::VerifyServiceServer;
use validblock::{AnchorRequest, VerifyRequest, Policy};

mod settings;
mod validblock;

use settings::SettingsStore;
use uuid::Uuid;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Default)]
struct AppState {
    bearer_token: String,
    trinity_mode: Mutex<bool>,
    settings: Mutex<SettingsStore>,
}

#[tauri::command]
async fn anchor_file(
    file_content: Vec<u8>,
    memo: String,
    use_on_chain: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    if use_on_chain && *state.trinity_mode.lock().unwrap() {
        return Err("Trinity mode is enabled. Cannot perform on-chain operations.".into());
    }

    if use_on_chain && memo.len() > 47 {
        return Err("Memo too long for on-chain anchor (max 47 bytes).".into());
    }

    let mut client = AnchorServiceClient::connect("http://127.0.0.1:50051")
        .await
        .map_err(|e| format!("gRPC connection failed: {}", e))?;

    let req = AnchorRequest {
        file_content,
        memo,
        policy: if use_on_chain {
            Policy::OnChain as i32
        } else {
            Policy::LocalOnly as i32
        },
    };

    let res = client
        .anchor(req)
        .await
        .map_err(|e| map_grpc_error(e.to_string()))?
        .into_inner();

    Ok(res.digest)
}

#[tauri::command]
async fn verify_file(
    file_content: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut client = VerifyServiceClient::connect("http://127.0.0.1:50051")
        .await
        .map_err(|e| format!("gRPC connection failed: {}", e))?;

    let req = VerifyRequest { file_content };

    let res = client
        .verify(req)
        .await
        .map_err(|e| e.to_string())?
        .into_inner();

    Ok(res.verified)
}

#[tauri::command]
fn toggle_trinity_mode(enable: bool, state: State<'_, AppState>) {
    *state.trinity_mode.lock().unwrap() = enable;
}

#[tauri::command]
fn get_trinity_mode(state: State<'_, AppState>) -> bool {
    *state.trinity_mode.lock().unwrap()
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> Result<SettingsStore, String> {
    Ok(state.settings.lock().unwrap().clone())
}

#[tauri::command]
fn put_settings(new_settings: SettingsStore, state: State<'_, AppState>) -> Result<(), String> {
    *state.settings.lock().unwrap() = new_settings;
    Ok(())
}

fn map_grpc_error(msg: String) -> String {
    if msg.contains("DuplicateDigest") {
        "Already anchored. Try again with a different file.".into()
    } else if msg.contains("InvalidMemo") {
        "Memo too long for on-chain anchor (max 47 bytes).".into()
    } else {
        msg
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let bearer_token = uuid::Uuid::new_v4().to_string();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            bearer_token,
            ..Default::default()
        })
        .invoke_handler(tauri::generate_handler![
            anchor_file,
            verify_file,
            toggle_trinity_mode,
            get_trinity_mode,
            get_settings,
            put_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
