#![allow(clippy::async_yields_async)]

use std::collections::HashMap;

use actix_web::web::Form;
use actix_web::{post, HttpResponse, Responder};

use tracing::instrument;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

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
        Err(_) => {
            let mut resp_body = HashMap::new();
            resp_body.insert("error", "Please try again later");
            HttpResponse::InternalServerError().json(resp_body)
        }
        Ok(id) => {
            let mut resp_body = HashMap::new();
            resp_body.insert("id", id.to_string());
            HttpResponse::Created().json(resp_body)
        }
    }
}

#[instrument(name = "SAVE SUBSCRIBER TO DATABASE", skip(_subscriber))]
async fn save_subscriber_to_db(_subscriber: &NewSubscriber) -> Result<Uuid, std::fmt::Error> {
    let id = Uuid::new_v4();
    Ok(id)
}
