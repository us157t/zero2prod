use secrecy::ExposeSecret;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{init_subscriber};

async fn hc() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber("zero2prod".to_string(), "info".to_string());
    let conf = get_configuration().expect("Failed to get conf");
    let addr = format!("127.0.0.1:{}", conf.application_port);
    let s = TcpListener::bind("127.0.0.1:0")?;
    let conn = PgPool::connect(&conf.database.connection_string().
	expose_secret()
	)
        .await
        .expect("Failed to conn postgres");
    dbg!(&addr);
    run(s, conn)?.await
}
