use std::collections::HashMap;
use actix_session::Session;
use actix_web::{web, Error, HttpRequest, HttpResponse, FromRequest};
use aes::cipher::consts::False;
use log::info;
use reqwest::{ header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE}, ClientBuilder, Method};
use serde_json::json;
use anyhow::anyhow;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use crate::{key_management::{encrypt_data, PasswordManager}, tools::handle_request_body};
use futures::StreamExt;

//-------------------Proxy handler image -----------------

pub async fn proxy_image_handler(
    req: HttpRequest,
    session: Session,
    password_manager: web::Data<PasswordManager>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("proxy request berjalan");
    
    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
        Ok(cookie) => cookie,
        Err(response) => return Ok(response),
    };

    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);

    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string().replace("image/", "");
    info!("url image {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:1015{}", url);

    // Membuat client
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    info!("creating client");
    // Menentukan metode request
    let mut request_builder = match req.method().as_str() {
        "GET" => client.get(&target_url),
        "POST" => client.post(&target_url),
        "PUT" => client.put(&target_url),
        "DELETE" => client.delete(&target_url), // Menambahkan metode DELETE
        _ => return Ok(HttpResponse::MethodNotAllowed().finish()),
    };

    // Menambahkan headers
    request_builder = request_builder.headers(headers);
    info!("set headers");
    // Untuk POST, PUT, dan DELETE, tambahkan body jika ada
    if req.method() == "POST" || req.method() == "PUT" || req.method() == "DELETE" {
        let content_type = req.headers().get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");
            info!("set headers 2");
        request_builder = match handle_request_body(content_type, payload, request_builder).await {
            Ok(builder) => builder,
            Err(response) => return Ok(response),
        };
        info!("request {:?}",request_builder);
    }

    // Kirim request
    info!("sending");
    match request_builder.send().await {
        Ok(response) => {
            let mut client_resp = HttpResponse::build(response.status());
            
            // Copy headers
            for (name, value) in response.headers() {
                if let Ok(value) = value.to_str() {
                    client_resp.header(name.clone(), value);
                }
            }

            // Get body
            let body = response.bytes().await.map_err(|e| {
                info!("sending error");
                actix_web::error::ErrorInternalServerError(format!("Failed to read response body: {}", e))
            })?;

            Ok(client_resp.body(body))
        },
        Err(e) => {
            info!("Gagal meneruskan permintaan: {}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e)))
        }
    }
}
fn create_encrypted_cookie(user_id: &str, password_manager: &PasswordManager) -> Result<String, HttpResponse> {
    let encryption_key = password_manager.get_password().map_err(|_| {
        HttpResponse::InternalServerError().body("Gagal mendapatkan kunci enkripsi")
    })?;

    let cookie_value = json!({ "user_id": user_id }).to_string();

    let encrypted_value = encrypt_data(cookie_value.as_bytes(), &encryption_key)
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Gagal mengenkripsi: {}", e)))?;

    let encrypted_string = general_purpose::STANDARD.encode(encrypted_value);

    Ok(format!("encrypted_cookie={}", encrypted_string))
}





pub async fn proxy_image_get_handler(
    req: HttpRequest,
    session: Session,
    password_manager: web::Data<PasswordManager>
) -> Result<HttpResponse, Error> {
    info!("proxy GET berjalan");
    
    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
        Ok(cookie) => cookie,
        Err(response) => return Ok(response),
    };
    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);

    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string().replace("image/", "");
    info!("url ai {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:1015{}", url);

    // Membuat client
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };



    // Forward permintaan GET ke target URL dengan headers yang sudah disiapkan
    match client
        .get(&target_url)
        .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            Ok(HttpResponse::Ok().body(body))   
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan: {}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))  )
        }
    }
}


pub async fn proxy_image_form_post_handler(
    data: web::Form<HashMap<String, String>>, // Menangkap form data
    req: HttpRequest,
    password_manager: web::Data<PasswordManager>,
    session: Session
) -> Result<HttpResponse, Error> {
    info!("proxy berjalan");
    

    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
        Ok(cookie) => cookie,
        Err(response) => return Ok(response),
    };
    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);


    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string().replace("image/", "");
    info!("url {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:1015{}", url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .form(&*data)       // Menyertakan form data dalam permintaan
        .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            Ok(HttpResponse::Ok().body(body))
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e)))
        }
    }
}
pub async fn proxy_image_json_post_handler(
    json_data: web::Json<Value>,
    req: HttpRequest,
    password_manager: web::Data<PasswordManager>,
    session: Session
) -> Result<HttpResponse, Error> {
    info!("proxy berjalan");
    

    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
        Ok(cookie) => cookie,
        Err(response) => return Ok(response),
    };
    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);


    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string().replace("image/", "");
    info!("url {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:1015{}", url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .json(&json_data.into_inner())       // Menyertakan form data dalam permintaan
        .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            Ok(HttpResponse::Ok().body(body))
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            Ok( HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e)))
        }
    }
}

pub async fn proxy_ai_nodata_post_handler(
    req: HttpRequest,
    password_manager: web::Data<PasswordManager>,
    session: Session
) -> Result<HttpResponse, Error> {
    info!("proxy berjalan");
    

    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
        Ok(cookie) => cookie,
        Err(response) => return Ok(response),
    };
    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);


    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string().replace("image/", "");
    info!("url {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:1015{}", url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .headers(headers)
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            Ok(HttpResponse::Ok().body(body))
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))) 
        }
    }
}