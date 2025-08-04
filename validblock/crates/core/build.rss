use std::path::{PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let proto_file = manifest_dir.join("../../proto/validblock.proto");
    let proto_include = manifest_dir.join("../../proto");
    let out_dir = manifest_dir.join("src/proto"); // ✅ this is valid because manifest_dir is a PathBuf

    std::fs::create_dir_all(&out_dir).expect("Failed to create src/proto directory");

    println!("cargo:rerun-if-changed={}", proto_file.display());
    println!("cargo:rerun-if-changed={}", proto_include.display());

    tonic_build::configure()
        .build_server(true)
        .out_dir(&out_dir)
        .compile(&[proto_file.to_str().unwrap()], &[proto_include.to_str().unwrap()])
        .expect("Failed to compile validblock.proto");
}
