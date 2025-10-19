use actix_web::web::{self, scope};

use crate::{ai_handlers::proxy_ai_handler, image_handlers::proxy_image_handler, user_handlers::{proxy_payment_handler, proxy_payment_no_login_handler, proxy_user_get_handler, proxy_user_post_handler, proxy_user_post_login_handler}};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            //.service(web::resource("/")
             //   .route(web::get().to(proxy_user_get_handler)))

            .service(web::resource("/")
                .route(web::get().to(proxy_ai_handler)))
            .service(web::resource("/html-editor")
                .route(web::get().to(proxy_ai_handler)))

            .service(web::resource("/login")
                .route(web::get().to(proxy_user_get_handler))
                .route(web::post().to(proxy_user_post_login_handler)))
            .service(web::resource("/privacy")
                .route(web::get().to(proxy_user_get_handler)))
            .service(web::resource("/terms")
                .route(web::get().to(proxy_user_get_handler)))
            .service(web::resource("/subscriptions")
                .route(web::get().to(proxy_user_get_handler)))
            .service(web::resource("/subscriptions_list")
                .route(web::get().to(proxy_user_get_handler)))
            .service(web::resource("/notif_payment")
                .route(web::post().to(proxy_payment_no_login_handler)))
            .service(web::resource("/payment")
                .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
                .route(web::post().to(proxy_payment_handler)))
            .service(web::resource("/check_payment")
                .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
                .route(web::post().to(proxy_payment_handler)))
            .service(web::resource("/show_all_saved_payment")
                .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
                .route(web::post().to(proxy_payment_handler)))
            .service(web::resource("/show_user_data")
                .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
                .route(web::post().to(proxy_payment_handler)))
            .service(web::resource("/cancel_payment")
                .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
                .route(web::post().to(proxy_payment_handler)))   
            .service(web::resource("/register")
                .route(web::get().to(proxy_user_get_handler))
                .route(web::post().to(proxy_user_post_handler)))
            .service(web::resource("/verify")
                .route(web::get().to(proxy_user_get_handler)))
            .service(web::resource("/auth/google/callback")
                .route(web::post().to(proxy_user_post_login_handler)))
            .service(web::resource("/e2324e2w34r3w4r3wer423w4r32w4234324/{tail:.*}")
                .route(web::get().to(proxy_ai_handler)))
           // .service(web::resource("/ai/image/{tail:.*}")
           //     .route(web::get().to(proxy_ai_nologin_handler)))

           // .service(web::resource("/chatbox")
            //    .wrap(actix_web_lab::middleware::from_fn(crate::middleware::check_auth_mw))
             //   .route(web::get().to(proxy_ai_handler)))


    );
}

pub fn configure_static_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/login-styles.css", web::get().to(proxy_user_get_handler));
    cfg.route("/subscriptions.css", web::get().to(proxy_user_get_handler));
    cfg.route("/payment.js", web::get().to(proxy_user_get_handler));    
    cfg.route("ai/{tail:.*}", web::get().to(proxy_ai_handler));
}

pub fn configure_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/ai/{tail:.*}", web::route().to(proxy_ai_handler));
    cfg.route("/image/{tail:.*}", web::route().to(proxy_image_handler));
}
