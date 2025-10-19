use std::io;
use std::fs;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};





pub const SERVER_ADDR: &str = "127.0.0.1:1001";
pub const CERT_FILE: &str = "tcm/fullchain.pem";
pub const KEY_FILE: &str = "tcm/privkey.pem";
pub const TEST_CASES_DIR: &str = "test_cases";

pub fn load_certs(filename: &str) -> io::Result<Vec<CertificateDer<'static>>> {
    let certfile = fs::File::open(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader).collect()
}

pub fn load_private_key(filename: &str) -> io::Result<PrivateKeyDer<'static>> {
    let keyfile = fs::File::open(filename)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);
    rustls_pemfile::private_key(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "failed to load private key"))
        .map(|key| key.unwrap())
}

pub fn get_exe_dir() -> io::Result<std::path::PathBuf> {
    std::env::current_exe()?
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get parent directory"))
        .map(|p| p.to_path_buf())
}