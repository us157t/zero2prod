use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;
    Ok(NewSubscriber { email, name })
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
    let new_subscriber = match _form.0.try_into() {
        Ok(_form) => _form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subs(&new_subscriber, &_conn).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving db!!!", skip(_form, _conn))]
pub async fn insert_subs(_form: &NewSubscriber, _conn: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4)
		"#,
        Uuid::new_v4(),
        _form.email.as_ref(),
        _form.name.as_ref(),
        Utc::now()
    )
    .execute(_conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
