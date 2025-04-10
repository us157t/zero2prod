use tracing_actix_web::TracingLogger;
use secrecy::ExposeSecret;
use crate::telemetry::init_subscriber;
use crate::configuration::get_configuration;
use crate::configuration::DatabaseSettings;
use crate::routes::{hc, subs};
use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use once_cell::sync::Lazy;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
	init_subscriber("test".into(), "debug".into());
});

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

pub fn run(lis: TcpListener, conn: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    dbg!(&lis);
    let s = HttpServer::new(move || {
        App::new()
	    .wrap(TracingLogger::default())
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
    Lazy::force(&TRACING);

    let lis = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = lis.local_addr().unwrap().port();
    let mut conf = get_configuration().expect("Failed to read conf");
    conf.database.database_name = Uuid::new_v4().to_string();
    let conn = configure_database(&conf.database).await;

    let s = run(lis, conn.clone()).expect("failed to bind addr");
    let _ = tokio::spawn(s);
    TestApp {
        addr: format!("http://127.0.0.1:{}", port),
        pool: conn,
    }
}
