use crate::email_client::EmailClient;
use crate::routes::{hc, subscribe};
use actix_web::dev::Server;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(lis: TcpListener, conn: PgPool, ec: EmailClient) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let ec = web::Data::new(ec);
    let s = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/hc", web::get().to(hc))
            .route("/subs", web::post().to(subscribe))
            .app_data(conn.clone())
            .app_data(ec.clone())
    })
    .listen(lis)?
    .run();
    Ok(s)
}
