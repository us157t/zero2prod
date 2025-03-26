use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::conf::get_configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber =
        telemetry::get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let conf = get_configuration().expect("Failed to read conf");
    //    let conn = PgPool::connect(&conf.database.conn_string().expose_secret())
    //        .await
    //        .expect("Failed to conn postgres");
    let conn = PgPoolOptions::new()
        //.connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(conf.database.with_db());
    let sender_email = conf
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        conf.email_client.base_url,
        sender_email,
        conf.email_client.token,
    );
    let addr = format!("{}:{}", conf.application.host, conf.application.port);
    let lis = TcpListener::bind(addr)?;
    run(lis, conn, email_client)?.await
}
