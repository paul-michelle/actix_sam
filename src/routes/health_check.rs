use actix_web::{get, http::header, HttpResponse, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
        .append_header(header::ContentType(mime::APPLICATION_JSON))
        .finish()
}
