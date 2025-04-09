use crate::routes::{hc, subs};
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use crate::configuration::get_configuration;
use sqlx::{PgPool, Connection};


pub fn run(lis: TcpListener,
	   conn: PgPool
) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let s = HttpServer::new(move || {
        App::new()
            .route("/hc", web::get().to(hc))
            .route("/subs", web::post().to(subs))
	    .app_data(conn.clone())
    })
    .listen(lis)?
    .run();
    Ok(s)
}

pub struct TestApp {
	pub addr: String,
	pub pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let lis = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = lis.local_addr().unwrap().port();
    let conf = get_configuration().expect("Failed to read conf");
    let conn = PgPool::connect(&conf.database.connection_string())
	.await
	.expect("Failed to conn to postgres");

    let s = run(lis, conn.clone()).expect("failed to bind addr");
    let _ = tokio::spawn(s);
    TestApp {
    addr: format!("http://127.0.0.1:{}", port),
    pool: conn
    }
}
