use actix_web::HttpResponse;
pub async fn hc() -> HttpResponse {
    HttpResponse::Ok().finish()
}
