use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::conf::get_configuration;
use zero2prod::startup::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conf = get_configuration().expect("Failed to read conf");
    let conn = PgPool::connect(&conf.database.conn_string())
        .await
        .expect("Failed to conn postgres");
    let addr = format!("127.0.0.1:{}", conf.application_port);
    let lis = TcpListener::bind(addr)?;
    run(lis, conn)?.await
}
