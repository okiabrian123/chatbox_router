use std::io;

use actix_session::SessionExt;
use actix_web::{body::BoxBody, dev::{ServiceRequest, ServiceResponse}, Error, HttpResponse};
use actix_web_lab::middleware::Next;
use log::info;

fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

pub(crate) async fn check_auth_mw(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    // Akses sesi dari request

    let session = req.get_session();
    let user_logged_in = session.get::<String>("user_id").unwrap_or(None).is_some();

    let user_id: Option<String> = session.get("user_id")?;

    if let Some(id) = user_id {
        // `user_id` ada, lanjutkan ke handler berikutnya
        info!("user id :  {id}" );
    } else {
        // `user_id` tidak ada, kirim respons Unauthorized
        info!("belum login");
    }

    
    if user_logged_in {
        // Lanjutkan jika sudah login
        Ok(next.call(req).await?)
    }else {
        // Clear the session data
        req.get_session().clear();

        // Invalidate the session cookie
        let response = HttpResponse::Found()
            .append_header(("Location", "/login")) // Redirect to the login page
            .append_header(("Set-Cookie", "username=; HttpOnly; Path=/; Max-Age=0;")) // Invalidate the session cookie
            .finish()
            .map_into_boxed_body();


        let ss = ServiceResponse::new(req.request().clone(), response);
        Ok(ss)
    } 
}