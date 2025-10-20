use actix_session::{SessionMiddleware, SessionExt, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web::HttpResponse;
use std::env;

/// Initialize Cookie session store for session management
pub async fn create_redis_session_store() -> Result<CookieSessionStore, Box<dyn std::error::Error>> {
    // Using CookieSessionStore as fallback since RedisActorSessionStore is not available
    // in the current version of actix-session
    Ok(CookieSessionStore::default())
}

/// Create session middleware with shared configuration
pub fn create_session_middleware(
    store: CookieSessionStore,
    secret_key: Key,
) -> SessionMiddleware<CookieSessionStore> {
    let cookie_domain = env::var("COOKIE_DOMAIN")
        .unwrap_or_else(|_| "localhost".to_string());
    
    let is_production = env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string()) == "production";

    SessionMiddleware::builder(store, secret_key)
        .cookie_name("login-session".to_string())
        .cookie_domain(Some(cookie_domain))
        .cookie_path("/".to_string())
        .cookie_secure(is_production) // Only secure in production
        .cookie_http_only(true)
        .cookie_same_site(actix_web::cookie::SameSite::Lax)
        .session_lifecycle(
            actix_session::config::PersistentSession::default()
                .session_ttl(actix_web::cookie::time::Duration::days(30))
        )
        .build()
}

/// Extract user ID from session with proper error handling
pub fn extract_user_id_from_session(req: &ServiceRequest) -> Result<String, Error> {
    let session = req.get_session();
    
    match session.get::<String>("user_id") {
        Ok(Some(user_id)) => Ok(user_id),
        Ok(None) => Err(actix_web::error::ErrorUnauthorized("User not logged in")),
        Err(e) => {
            log::error!("Session error: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    }
}

/// Check if user is authenticated
pub fn is_user_authenticated(req: &ServiceRequest) -> Result<bool, Error> {
    let session = req.get_session();
    
    match session.get::<String>("user_id") {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Session error")),
    }
}

/// Set user session after successful login
pub fn set_user_session(
    session: &actix_session::Session,
    user_id: &str,
    username: &str,
) -> Result<(), Error> {
    session.insert("user_id", user_id)
        .map_err(|e| {
            log::error!("Failed to set user_id in session: {}", e);
            actix_web::error::ErrorInternalServerError("Session error")
        })?;
    
    session.insert("username", username)
        .map_err(|e| {
            log::error!("Failed to set username in session: {}", e);
            actix_web::error::ErrorInternalServerError("Session error")
        })?;
    
    // Set session expiration
    session.renew();
    
    Ok(())
}

/// Clear user session on logout
pub fn clear_user_session(session: &actix_session::Session) -> Result<(), Error> {
    session.clear();
    Ok(())
}

/// Refresh session to extend expiration
pub fn refresh_session(session: &actix_session::Session) -> Result<(), Error> {
    session.renew();
    Ok(())
}

/// Get session information for debugging
pub fn get_session_info(session: &actix_session::Session) -> Result<serde_json::Value, Error> {
    let user_id: Option<String> = session.get("user_id").unwrap_or(None);
    let username: Option<String> = session.get("username").unwrap_or(None);
    
    Ok(serde_json::json!({
        "user_id": user_id,
        "username": username,
        "has_session": user_id.is_some()
    }))
}

/// Create encrypted cookie for downstream service communication
pub fn create_encrypted_cookie(user_id: &str, password_manager: &crate::key_management::PasswordManager) -> Result<String, HttpResponse> {
    let encryption_key = password_manager.get_password().map_err(|_| {
        HttpResponse::InternalServerError().body("Gagal mendapatkan kunci enkripsi")
    })?;

    let cookie_value = serde_json::json!({ "user_id": user_id }).to_string();

    let encrypted_value = crate::key_management::encrypt_data(cookie_value.as_bytes(), &encryption_key)
        .map_err(|e| HttpResponse::InternalServerError().body(format!("Gagal mengenkripsi: {}", e)))?;

    let encrypted_string = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, encrypted_value);

    Ok(format!("encrypted_cookie={}", encrypted_string))
}