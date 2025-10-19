use std::{collections::HashMap, fmt::format};
use actix_session::Session;
use actix_web::{web::{self, Data}, Error, FromRequest, HttpRequest, HttpResponse};
use aes::cipher::consts::False;
use log::info;
use reqwest::{ header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE}, ClientBuilder, Method};
use serde_json::json;
use crate::Args;
use anyhow::anyhow;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use crate::{key_management::{encrypt_data, PasswordManager}, tools::{handle_request_body, run_server}};
use futures::StreamExt;
use bytes::Bytes;
use futures::stream::{self, TryStreamExt};
use reqwest::Response;
use std::process::Command;
//-------------------Proxy handler User -----------------

pub async fn proxy_user_post_handler(
    data: web::Form<HashMap<String, String>>, // Menangkap form data
    args: Data<Args>,
    req: HttpRequest
) -> HttpResponse {
    info!("proxy auth berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string();
    info!("url {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:{}1{}",args.port, url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .form(&*data)       // Menyertakan form data dalam permintaan
        .send()
        .await
    {
        Ok(response) => {
            info!("Menerima respons dari target {}",response.status());
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            run_server("chatbox-user_handlers".to_string(),format!("--ip {} --port {}1",args.ip,args.port).to_string());
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))

        }
    }
}
pub async fn proxy_user_post_login_handler(session: Session,
    data: web::Form<HashMap<String, String>>, // Menangkap form data
    args: Data<Args>,
    req: HttpRequest
) -> HttpResponse {
    info!("proxy berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string();
    info!("url {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:{}1{}",args.port, url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .form(&*data)       // Menyertakan form data dalam permintaan
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
          
            // info!("Menerima respons dari target {}", body);

            let check= response.status().is_success();
                

            let body = match response.text().await {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Kesalahan membaca body respons: {}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            
            if check {
                let response_json: serde_json::Value = match serde_json::from_str(&body) {
                    Ok(json) => json,
                    Err(_) => json!({ "message": "Tidak dapat mengurai respons JSON." }),
                };
                
                 // Jika login berhasil, set session dan API key
                     // Safely extract the "status" field as a boolean
                let status = response_json
                .get("success")  // Get the "status" field
                .and_then(|s| s.as_bool())  // Convert to Option<bool>
                .unwrap_or(false);  // Provide a default value (false) if not found or not a bool


                 info!("response_json {}", response_json);
                info!("status {}", status);
                if status {
                    let username = response_json
                    .get("username")  // Get the "message" field
                    .and_then(|m| m.as_str())  // Convert to &str, returns Option<&str>
                    .unwrap_or("");  // Provide a default value if "message" is not found or is not a string
                    
                    let _ = session.insert("user_id", username);
                    //let apikey = generate_apikey("172.", form.username.clone().as_str());
                    
                     let user_id: Option<String> =  match session.get("user_id") {
                         Ok(id) => id,
                         Err(_) => None,
                     };
                    
                      if let Some(id) = user_id {
                          info!("User ID: {id}");
                      } else {
                        info!("Belum login");
                      }
                }

            }else{
                info!("Belum login2");
            }
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            run_server("chatbox-user_handlers".to_string(),format!("--ip {} --port {}1",args.ip,args.port).to_string());
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))
            
        }
    }
}

pub async fn proxy_user_get_handler(
    args: Data<Args>,
    req: HttpRequest
) -> HttpResponse {
    info!("proxy GET berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string();
    info!("url data {}", url);

    // Membuat client dengan opsi untuk mengabaikan validasi sertifikat
    let client = match ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Kesalahan membangun klien");
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Menyusun target URL (local Actix Web server pada port yang berbeda)
    let target_url = format!("https://127.0.0.1:{}1{}",args.port, url);
    info!("target_url {}", target_url);
    // Forward permintaan GET ke target URL
    match client
        .get(&target_url)  // Menggunakan GET untuk mengirim permintaan
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
             info!("Menerima respons dari target {}", body);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            run_server("chatbox-user_handlers".to_string(),format!("--ip {} --port {}1",args.ip,args.port).to_string());
            // Mengembalikan pesan error jika forward request gagal
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))
        }
    }
}



pub async fn proxy_payment_handler(
    args: Data<Args>,
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
    let url = req.uri().path().to_string().replace("", "");
    info!("url payment {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:{}1{}",args.port, url);

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
            run_server("chatbox-user_handlers".to_string(),format!("--ip {} --port {}1",args.ip,args.port).to_string());
            info!("Gagal meneruskan permintaan: {}", e);
            Ok(HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e)))
        }
    }
}

pub async fn proxy_payment_no_login_handler(
    args: Data<Args>,
    req: HttpRequest,
    session: Session,
    password_manager: web::Data<PasswordManager>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    info!("proxy request berjalan");
    


    // Menyiapkan headers untuk request
    let mut headers = HeaderMap::new();
    

    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().path().to_string().replace("", "");
    info!("url payment {}", url);

    // Menyusun target URL
    let target_url = format!("https://127.0.0.1:{}1{}",args.port, url);

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
            run_server("chatbox-user_handlers".to_string(),format!("--ip {} --port {}1",args.ip,args.port).to_string());
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
