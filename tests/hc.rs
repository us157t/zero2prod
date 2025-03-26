use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::postgres::PgSslMode;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::conf::get_configuration;
use zero2prod::conf::DatabaseSettings;
use zero2prod::email_client::EmailClient;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn hc_works() {
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    let res = cli
        .get(format!("{}/hc", &app.address))
        .send()
        .await
        .expect("Failed to exe req!!!!!!!!!!!!!!!!!");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);
    let lis = TcpListener::bind("127.0.0.1:0").expect("Failed to lis!!!");
    let port = lis.local_addr().unwrap().port();

    let mut conf = get_configuration().expect("Failed to get conf");
    conf.database.database_name = Uuid::new_v4().to_string();
    let conn = conf_db(&conf.database).await;
    let sender_email = conf
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let email_client = EmailClient::new(
        conf.email_client.base_url,
        sender_email,
        conf.email_client.token,
    );
    let s = run(lis, conn.clone(), email_client).expect("Failed to run");
    let _ = tokio::spawn(s);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool: conn,
    }
}

#[tokio::test]
async fn subs_200() {
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = cli
        .post(&format!("{}/subs", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch data");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subs_400() {
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let res = cli
            .post(&format!("{}/subs", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            res.status().as_u16(),
            "The API did not fail with 400 Bad Request {}",
            error_message
        );
    }
}

pub async fn conf_db(conf: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect_with(&conf.without_db())
        .await
        .expect("Failed to conf_db");

    conn.execute(
        format!(
            r#"
				CREATE DATABASE "{}";
			"#,
            conf.database_name
        )
        .as_str(),
    )
    .await
    .expect("Failed to create database");

    let conn = PgPool::connect_with(conf.with_db())
        .await
        .expect("Failed to conn to postgres");

    sqlx::migrate!("./migrations")
        .run(&conn)
        .await
        .expect("Failed to migrate the db");

    conn
}

#[tokio::test]
async fn subscribe_returns_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        //("name=xx&email=ursula_le_guin%40gmail.com", "empty name11111"),
        //("name=Ursula&email=xx", "empty email222222"),
        ("name=Ursula&email=email.com", "invalid email66666666666"),
    ];

    for (body, description) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subs", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            200,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        );
    }
}
