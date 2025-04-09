use actix_web::{HttpResponse, Responder};
pub async fn hc() -> impl Responder {
    HttpResponse::Ok()
}
