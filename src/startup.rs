use crate::routes::{hc, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(lis: TcpListener, conn: PgPool) -> Result<Server, std::io::Error> {
    let conn = web::Data::new(conn);
    let s = HttpServer::new(move || {
        App::new()
            .route("/hc", web::get().to(hc))
            .route("/subs", web::post().to(subscribe))
            .app_data(conn.clone())
    })
    .listen(lis)?
    .run();
    Ok(s)
}
