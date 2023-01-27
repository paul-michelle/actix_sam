use std::net::TcpListener;

use actix_sam::settings::{figure_environment, get_settings};
use actix_sam::startup::run;
use actix_sam::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("actix_sam".into(), "actix_sam=info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let environ = figure_environment();
    let settings = get_settings(&environ);
    let app_addr = format!("{}:{}", settings.app.host, settings.app.port);
    let listener = TcpListener::bind(app_addr).expect("Failed to bind address");
    let launched_srv = run(listener).await.expect("Failed to launch server.");

    launched_srv.await
}
