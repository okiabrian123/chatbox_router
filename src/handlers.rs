use std::collections::HashMap;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use aes::cipher::consts::False;
use log::info;
use reqwest::ClientBuilder;
use serde_json::json;

// proxy handler AI
pub async fn proxy_user_post_handler(
    data: web::Form<HashMap<String, String>>, // Menangkap form data
    req: HttpRequest
) -> HttpResponse {
    info!("proxy berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string();
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
    let target_url = format!("https://127.0.0.1:1005{}", url);

    // Forward permintaan dengan form data ke target URL
    match client
        .post(&target_url)  // Menggunakan POST untuk mengirim form data
        .form(&*data)       // Menyertakan form data dalam permintaan
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))
        }
    }
}
pub async fn proxy_user_post_login_handler(session: Session,
    data: web::Form<HashMap<String, String>>, // Menangkap form data
    req: HttpRequest
) -> HttpResponse {
    info!("proxy berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string();
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
    let target_url = format!("https://127.0.0.1:1005{}", url);

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
                .get("status")  // Get the "status" field
                .and_then(|s| s.as_bool())  // Convert to Option<bool>
                .unwrap_or(false);  // Provide a default value (false) if not found or not a bool

                 let username = response_json
                 .get("message")  // Get the "message" field
                 .and_then(|m| m.as_str())  // Convert to &str, returns Option<&str>
                 .unwrap_or("");  // Provide a default value if "message" is not found or is not a string

                if status {
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
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))
        }
    }
}

pub async fn proxy_user_get_handler(
    req: HttpRequest
) -> HttpResponse {
    info!("proxy GET berjalan");
    
    // Mengambil URL dari request dan memodifikasi
    let url = req.uri().to_string();
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
    let target_url = format!("https://127.0.0.1:1005{}", url);

    // Forward permintaan GET ke target URL
    match client
        .get(&target_url)  // Menggunakan GET untuk mengirim permintaan
        .send()
        .await
    {
        Ok(response) => {
            // Mengambil isi body dari response target dan forward kembali ke client
            let body = response.text().await.unwrap_or_else(|_| "Kesalahan membaca body respons".into());
            // info!("Menerima respons dari target {}", body);
            HttpResponse::Ok().body(body)
        }
        Err(e) => {
            info!("Gagal meneruskan permintaan");
            // Mengembalikan pesan error jika forward request gagal
            HttpResponse::InternalServerError().body(format!("Gagal meneruskan permintaan: {}", e))
        }
    }
}