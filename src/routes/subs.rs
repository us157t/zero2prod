use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


#[tracing::instrument(
	name = "Add a new subs 2025-03-20",
	skip(_form, _conn),
	fields(
	    subscriber_email = %_form.email,
	    subscriber_name= %_form.name
	)
)]
pub async fn subscribe(_form: web::Form<FormData>, _conn: web::Data<PgPool>) -> HttpResponse {
    match  insert_subs(&_form, &_conn).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
     name = "Saving db!!!",
     skip(_form, _conn)
)]
pub async fn insert_subs(
     _form: &FormData,
     _conn: &PgPool)
 -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    ).execute(_conn)
     .await
     .map_err(|e| {
	tracing::error!("Failed to execute query: {:?}", e);
	e
	})?;
	Ok(())
}
