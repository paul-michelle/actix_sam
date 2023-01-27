use std::net::TcpListener;

use actix_sam::settings::{figure_environment, get_settings};
use actix_sam::startup::run;
use actix_sam::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = figure_environment();
    let settings = get_settings(&environment);

    let subscriber = get_subscriber("actix_sam".into(), "actix_sam=info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))
    .expect("Failed to bind address");

    run(listener).await?.await
}
