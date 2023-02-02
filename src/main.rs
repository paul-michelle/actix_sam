use actix_sam::settings::get_local_settings;
use actix_sam::startup::{run, run_on_lambda};
use actix_sam::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("actix_sam".into(), "actix_sam=info".into(), std::io::stdout);
    init_subscriber(subscriber);
    
    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        run_on_lambda().await;
        return Ok(());
    }

    let settings = get_local_settings();
    let app_addr = format!("{}:{}", settings.app.host, settings.app.port);
    let listener = TcpListener::bind(app_addr).expect("Failed to bind address");
    let launched_srv = run(listener)
        .await
        .expect("Failed to launch server locally.");
    launched_srv.await
}
