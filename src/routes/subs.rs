use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::{Connection, PgPool};
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(_form, conn),
fields(
Subscriber_email = %_form.email,
Subscriber_name= %_form.name
)
)]
pub async fn subs(_form: web::Form<FormData>, conn: web::Data<PgPool>) -> impl Responder {
    sqlx::query!(
        r#"
	INSERT INTO subscriptions (id, email, name, subscribed_at)
	VALUES ($1, $2, $3, $4)
	"#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(conn.get_ref())
    .await;
    tracing::info!("Wowi222!!!! {}", Uuid::new_v4());
    HttpResponse::Ok()
}
