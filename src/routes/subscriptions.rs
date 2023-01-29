#![allow(clippy::async_yields_async)]

use lambda_web::actix_web::web::Form;
use lambda_web::actix_web::{post, HttpResponse, Responder};

use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{
    ErrorResponse, NewSubscriber, SubscriberEmail, SubscriberExposed, SubscriberName,
};

#[derive(serde::Deserialize)]
struct SubsciptionForm {
    name: String,
    email: String,
}

impl TryFrom<SubsciptionForm> for NewSubscriber {
    type Error = String;

    fn try_from(value: SubsciptionForm) -> Result<Self, Self::Error> {
        let name = SubscriberName::from_string(value.name)?;
        let email = SubscriberEmail::from_string(value.email)?;
        Ok(NewSubscriber { name, email })
    }
}

#[instrument(
    name = "REGISTER NEW SUBSCRIBER",
    skip(form),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscriptions")]
async fn subscribe(form: Form<SubsciptionForm>) -> impl Responder {
    let new_subscriber = match form.0.try_into() {
        Ok(subscriber) => subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match save_subscriber_to_db(&new_subscriber).await {
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse::with_message(
            "Please try again later.".to_string(),
        )),
        Ok(details) => HttpResponse::Created().json(details),
    }
}

#[instrument(name = "SAVE SUBSCRIBER TO DATABASE", skip(_subscriber))]
async fn save_subscriber_to_db(
    _subscriber: &NewSubscriber,
) -> Result<SubscriberExposed, std::fmt::Error> {
    let id = Uuid::new_v4();
    let created_at = Utc::now();
    Ok(SubscriberExposed { id, created_at })
}
