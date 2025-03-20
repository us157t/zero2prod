use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::conf::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read conf");
    let conn = PgPool::connect(&conf.database.conn_string().expose_secret())
        .await
        .expect("Failed to conn postgres");
    let addr = format!("127.0.0.1:{}", conf.application_port);
    let lis = TcpListener::bind(addr)?;
    run(lis, conn)?.await
}
