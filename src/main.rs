use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::{PgConnection, Connection};
use sqlx::PgPool;
async fn hc() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration().expect("Failed to get conf");
    let addr = format!("127.0.0.1:{}", conf.application_port);
    let s = TcpListener::bind("127.0.0.1:0")?;
    let conn = PgPool::connect(&conf.database.connection_string())
	.await
	.expect("Failed to conn postgres");
    run(s, conn)?.await
}
