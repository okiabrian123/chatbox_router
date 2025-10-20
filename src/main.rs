use actix_web::{web, App, HttpServer, HttpResponse};
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_cors::Cors;

use key_management::PasswordManager;
use log::info;
use rustls::{crypto::CryptoProvider, ServerConfig};
use std::{collections::HashMap, env, fs, io::{self, BufRead}, path::{Path, PathBuf}};

mod middleware;
mod routes;
mod config;
mod user_handlers;
mod key_management;
mod tools;
mod ai_handlers;
mod image_handlers;
mod session_config;
use keyring::Entry;
use std::error::Error;
use middleware::check_auth_mw;
use routes::{configure_api_routes, configure_routes, configure_static_routes};
use config::{load_certs, load_private_key};
use session_config::{create_redis_session_store, create_session_middleware};

fn initialize_password_manager(service_name: &str, username: &str, key: &str) -> web::Data<PasswordManager> {
    info!("test0");
    let entry = match Entry::new("aaaa", "bbbb") {
        Ok(entry) => entry,
        Err(e) => {
            eprintln!("Gagal membuat Entry: {}", e);
            std::process::exit(1);
        }
    };
    info!("test1");
    let password_manager = web::Data::new(match PasswordManager::new(service_name, username, key) { 
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Gagal membuat PasswordManager: {}", e);
            std::process::exit(1);
        }
    });
    info!("test2");
    // Menghapus kredensial yang ada jika ada
    if !password_manager.get_password().is_err() {
        password_manager.delete_credential().expect("Gagal menghapus kredensial");
    }

    // Membuat PasswordManager baru setelah menghapus kredensial
    let password_manager = web::Data::new(match PasswordManager::new(service_name, username, key) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Gagal membuat PasswordManager: {}", e);
            std::process::exit(1);
        }
    });

    // Menghasilkan dan menyimpan kunci enkripsi baru
    let new_encryption_key = match password_manager.generate_random_key(32) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Gagal menghasilkan kunci enkripsi acak: {}", e);
            std::process::exit(1);
        }
    };
    info!("new_encryption_key length {}", new_encryption_key.len());
    password_manager.set_password(&new_encryption_key)
        .expect("Gagal menyimpan kunci enkripsi");
    println!("Kunci enkripsi baru berhasil dihasilkan dan disimpan.");


    let test = password_manager.get_password().expect("Gagal mendapatkan kunci enkripsi");
info!("test length {}", test.len());
    password_manager
}

use clap::Parser;

/// Program untuk demo CLI args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
/// IP address to bind the server to
#[arg(short, long)]
pub ip: String,

/// Port number to bind the server to
#[arg(short, long)]
pub port: u16,

}
#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    rustls::crypto::ring::default_provider().install_default().expect("Gagal memasang penyedia crypto rustls");
    env_logger::init();
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get parent directory");
    let file_path = ".config/px.toml";

    // Periksa apakah file ada
    if !Path::new(file_path).exists() {
        eprintln!("ada bug bang-1");
        return Ok(());
    }
  // Baca file dan parsing isi
    let credentials = read_credentials(file_path)?;
    println!("hapus file tidak gunakan agar efisien");
    fs::remove_file(file_path)?;

    let exe_dir = env::current_exe()?.parent().expect("Failed to get parent directory").to_path_buf();

    let certs_path = exe_dir.join("certs/mycert.pem");
    let certs_path_s=match certs_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Gagal mengambil path cert");
            std::process::exit(1);
        }
    };
    let key_path =  exe_dir.join("certs/mykey.pem");
    let key_path_s=match key_path.to_str() {
        Some(path) => path,
        None => {
            eprintln!("Gagal mengambil path key");
            std::process::exit(1);
        }
    };

    let secret_key = Key::generate();

    let certs = load_certs(certs_path_s)?;
    let key = load_private_key(key_path_s)?;
    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let sn = credentials.get("sn").expect("bug bang-2").to_string();
    let us = credentials.get("us").expect("bug bang-3").to_string();
    let ky = credentials.get("ky").expect("bug bang-4").to_string();
    let password_manager = initialize_password_manager(&sn, &us, &ky);
    let ip = args.ip.clone();
    let port = args.port.clone();
    let args_data = web::Data::new(args);
    // Create Redis session store for shared session management
    let session_store = match create_redis_session_store().await {
        Ok(store) => store,
        Err(e) => {
            eprintln!("Failed to create Redis session store: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to create session store"));
        }
    };

    HttpServer::new(move || {
        App::new()
        .app_data(password_manager.clone())
        .app_data(args_data.clone())
        .wrap(
            Cors::default()
                .allowed_origin("https://testcasemanager.my.id") // Allow specific origin
                .allowed_origin("https://chatpintar.my.id") // Allow specific origin
                .allowed_origin("https://local3.testcasemanager.my.id") // Allow specific origin
                .allowed_origin("http://localhost:8080") // Allow local development
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec!["Content-Type", "Authorization", "Cookie"])
                .max_age(3600),
        )
            .wrap(create_session_middleware(session_store.clone(), secret_key.clone()))
                .service(web::scope("/static").configure(configure_static_routes))
                .service(
                    web::scope("/api")
                        .wrap(actix_web_lab::middleware::from_fn(check_auth_mw))
                        .configure(configure_api_routes)
                )
                .service(web::resource("/health")
                    .route(web::get().to(health_check)))
                .configure(configure_routes)
    })
    .bind_rustls_0_23(format!("{}:{}", ip, port) , server_config)?
    .run()
    .await
}
fn read_credentials(file_path: &str) -> io::Result<HashMap<String, String>> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut credentials = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once('=') {
            credentials.insert(key.trim().to_string(), value.trim().to_string());
        }
    }


// Health check endpoint for Docker container
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "proxy_handler",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
    Ok(credentials)
}