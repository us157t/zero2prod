use sqlx::{Connection, PgConnection};
use zero2prod::configuration::fmt;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::spawn_app;

#[tokio::test]
async fn hc() {
    let addr = spawn_app().await;
    let cli = reqwest::Client::new();
    let res = cli
        .get(format!("{}/hc", addr.addr))
        .send()
        .await
        .expect("failed 222");

    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn _200() {
    let app = spawn_app().await;
    let cli = reqwest::Client::new();
    dbg!(&app.addr);

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let res = cli
        .post(format!("{}/subs", &app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute req");
    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
