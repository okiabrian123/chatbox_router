use std::{collections::HashMap, fs::File, process::{Command, Stdio}};
use actix_web::{web, HttpResponse};
use serde_json::Value;
use crate::key_management::{encrypt_data, PasswordManager};
use futures::StreamExt;
use reqwest::multipart::{Form, Part};
use bytes::{Bytes, BytesMut};
use log::info;
pub async fn handle_request_body(
    content_type: &str,
    mut payload: web::Payload,
    request_builder: reqwest::RequestBuilder
) -> Result<reqwest::RequestBuilder, HttpResponse> {
    match content_type {
        ct if ct.starts_with("application/json") => {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.map_err(|e| HttpResponse::BadRequest().body(format!("Error reading payload: {}", e)))?;
                body.extend_from_slice(&chunk);
            }
            let json: Value = serde_json::from_slice(&body)
                .map_err(|e| HttpResponse::BadRequest().body(format!("Invalid JSON: {}", e)))?;
            Ok(request_builder.json(&json))
        },
        ct if ct.starts_with("application/x-www-form-urlencoded") => {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk = chunk.map_err(|e| HttpResponse::BadRequest().body(format!("Error reading payload: {}", e)))?;
                body.extend_from_slice(&chunk);
            }
            let form_str = String::from_utf8(body.to_vec())
                .map_err(|e| HttpResponse::BadRequest().body(format!("Invalid UTF-8 sequence: {}", e)))?;
            let form_data: HashMap<String, String> = form_str.split('&')
                .filter_map(|item| {
                    let mut parts = item.splitn(2, '=');
                    Some((
                        parts.next()?.to_string(),
                        parts.next().unwrap_or("").to_string(),
                    ))
                })
                .collect();
            Ok(request_builder.form(&form_data))
        },
        ct if ct.starts_with("multipart/form-data") => {
            info!("Forwarding multipart form data directly");
            
            // Collect payload into bytes
            let mut body = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                let chunk: Bytes = chunk.map_err(|e| {
                    info!("Error reading payload chunk: {}", e);
                    HttpResponse::InternalServerError().body(format!("Failed to read payload: {}", e))
                })?;
                body.extend_from_slice(&chunk);
            }
            // Forward the raw bytes with the original content-type
            info!("Forwarding raw multipart data");
            Ok(request_builder.header("content-type", content_type).body(body.freeze()))
        },
        _ => Err(HttpResponse::UnsupportedMediaType().finish()),
    }
}
pub fn run_server(apps:String,args:String)->std::io::Result<()>{
    //let log_file = File::create(format!("program-{}.log",apps))?;
   // 
   // Command::new("sh")
    //    .arg("-c")
    //    .arg(format!("nohup ./{} {} > program-{}.log 2>&1 & disown",apps,args,apps))
     //   .spawn()?
      //  .wait()?; // Tidak perlu disown karena sudah di-handle oleh shell
    Ok(())
}

