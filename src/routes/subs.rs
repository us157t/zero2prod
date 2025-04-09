use chrono::Utc;
use uuid::Uuid;
use sqlx::{PgPool, Connection};
use actix_web::{web, HttpResponse, Responder};
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subs(_form: web::Form<FormData>,
		  conn: web::Data<PgPool>,
) -> impl Responder {
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
    HttpResponse::Ok()
}
