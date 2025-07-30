#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use validblock_types::{Digest256, VBError};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB

/// Hash a file at the given path, rejecting symlinks.
pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<Digest256, VBError> {
	let meta = std::fs::symlink_metadata(&path)?;
	if meta.file_type().is_symlink() {
		return Err(VBError::Other("Symlinks are not allowed".into()));
	}
	let file = File::open(path)?;
	hash_reader(file)
}

/// Hash any reader, streaming in 1 MiB chunks.
pub fn hash_reader<R: Read>(mut reader: R) -> Result<Digest256, VBError> {
	let mut hasher = Sha256::new();
	let mut buf = vec![0u8; CHUNK_SIZE];
	loop {
		let n = reader.read(&mut buf)?;
		if n == 0 {
				break;
		}
		hasher.update(&buf[..n]);
	}
	let result = hasher.finalize();
	Ok(Digest256(result.into()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::io::Write;
	use tempfile::tempdir;

  #[test]
	fn test_hash_known_vector_file() {
		let path = std::path::Path::new("tests/data/1MB.bin");
		if !path.exists() {
			eprintln!("SKIP: tests/data/1MB.bin not found");
			return;
		}
		let expected = "3e3af97b73845ee73bf7cdce5c9035e4ebd4f9f6eb09551ba7621e1f0a1029b7";
		let digest = hash_file(path).unwrap();
		assert_eq!(digest.to_string(), expected);
	}

	#[test]
	fn test_hash_large_random_file() {
		let dir = tempdir().unwrap();
		let file_path = dir.path().join("large.bin");
		let mut file = File::create(&file_path).unwrap();
		let data = vec![42u8; 32 * 1024 * 1024];
		file.write_all(&data).unwrap();
		file.sync_all().unwrap();

		let digest = hash_file(&file_path).unwrap();
		let digest2 = hash_reader(&data[..]).unwrap();
		assert_eq!(digest, digest2);
	}

	#[test]
	fn test_symlink_rejection() {
		let dir = tempdir().unwrap();
		let file_path = dir.path().join("file.txt");
		let symlink_path = dir.path().join("link.txt");
		fs::write(&file_path, b"hello").unwrap();
		#[cfg(unix)]
		std::os::unix::fs::symlink(&file_path, &symlink_path).unwrap();
		#[cfg(windows)]
		std::os::windows::fs::symlink_file(&file_path, &symlink_path).unwrap();
		let err = hash_file(&symlink_path).unwrap_err();
		match err {
			VBError::Other(msg) => assert!(msg.contains("Symlinks")),
			_ => panic!("Expected symlink error"),
		}
	}
}