use std::collections::HashMap;
use actix_session::Session;
use actix_web::{web::{self, Data}, Error, FromRequest, HttpRequest, HttpResponse};
use aes::cipher::consts::False;
use log::info;
use reqwest::{ header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE}, ClientBuilder, Method};
use serde_json::json;
use anyhow::anyhow;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use crate::{key_management::{encrypt_data, PasswordManager}, tools::{handle_request_body, run_server}, Args};
use futures::StreamExt;
use bytes::Bytes;
use futures::stream::{self, TryStreamExt};
use reqwest::Response;

//-------------------Proxy handler ai -----------------

pub async fn proxy_ai_handler(
    args: Data<Args>,
    req: HttpRequest,
    session: Session,
    password_manager: web::Data<PasswordManager>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("proxy request berjalan");
    
    let user_id = match session.get::<String>("user_id") {
        Ok(Some(id)) => id,
        _ => "none".to_string(),
    };


    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    
    if(user_id != "none") {
        
        let encrypted_cookie = match create_encrypted_cookie(&user_id, &password_manager) {
            Ok(cookie) => cookie,
            Err(response) => return Ok(response),
        };
        headers.insert(COOKIE, HeaderValue::from_str(&encrypted_cookie)?);
    }


    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string().replace("ai/", "");
    info!("url ai {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:{}3{}",args.port, url);

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

    // Menentukan metode request
    let mut request_builder = match req.method().as_str() {
        "GET" => client.get(&target_url),
        "POST" => client.post(&target_url),
        "PUT" => client.put(&target_url),
        "DELETE" => client.delete(&target_url), 
        _ => return Ok(HttpResponse::MethodNotAllowed().finish()),
    };

    // Menambahkan headers
    request_builder = request_builder.headers(headers);

    // Untuk POST, PUT, dan DELETE, tambahkan body jika ada
    if req.method() == "POST" || req.method() == "PUT" || req.method() == "DELETE" {
        let content_type = req.headers().get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        request_builder = match handle_request_body(content_type, payload, request_builder).await {
            Ok(builder) => builder,
            Err(response) => return Ok(response),
        };
    }

    // Kirim request
    match request_builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response.headers().clone();

            // Create response builder with status
            let mut client_resp = HttpResponse::build(status);
            
            // Copy headers
            for (name, value) in headers.iter() {
                if let Ok(value) = value.to_str() {
                    client_resp.header(name.clone(), value);
                }
            }

            // Create streaming body
            let stream = stream::try_unfold(response, |mut response| async move {
                match response.chunk().await {
                    Ok(Some(chunk)) => Ok(Some((chunk, response))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
                }
            });
            Ok(client_resp.streaming(stream))
        },
        Err(e) => {
            info!("Gagal meneruskan permintaan: {}", e);
            run_server("chatbox-API_AI".to_string(),format!("--ip {} --port {}3",args.ip,args.port).to_string());
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



pub async fn proxy_ai_nologin_handler(
    args: Data<Args>,
    req: HttpRequest,
    session: Session,
    password_manager: web::Data<PasswordManager>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("proxy request berjalan");
    

    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string().replace("ai/", "");
    info!("url ai {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:{}3{}",args.port, url);

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

    // Menentukan metode request
    let mut request_builder = match req.method().as_str() {
        "GET" => client.get(&target_url),
        "POST" => client.post(&target_url),
        "PUT" => client.put(&target_url),
        "DELETE" => client.delete(&target_url), 
        _ => return Ok(HttpResponse::MethodNotAllowed().finish()),
    };
    
    // Untuk POST, PUT, dan DELETE, tambahkan body jika ada
    if req.method() == "POST" || req.method() == "PUT" || req.method() == "DELETE" {
        let content_type = req.headers().get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("");

        request_builder = match handle_request_body(content_type, payload, request_builder).await {
            Ok(builder) => builder,
            Err(response) => return Ok(response),
        };
    }

    // Kirim request
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
                actix_web::error::ErrorInternalServerError(format!("Failed to read response body: {}", e))
            })?;

            Ok(client_resp.body(body))
        },
        Err(e) => {
            info!("Gagal meneruskan permintaan: {}", e);
            run_server("chatbox-API_AI".to_string(),format!("--ip {} --port {}3",args.ip,args.port).to_string());
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e)))
        }
    }
}