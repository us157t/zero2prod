use zero2prod::run;
use actix_web::{web, App,HttpResponse, HttpRequest, HttpServer, Responder};

async fn hc() -> impl Responder {
	HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	run()?.await
}
