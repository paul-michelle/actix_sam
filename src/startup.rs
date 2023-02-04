use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub enum APIStage {
    Dev,
    Prod,
}

impl APIStage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "Dev",
            Self::Prod => "Prod",
        }
    }
}

// https://actix.rs/docs/application/
fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check).service(subscribe);
}

pub async fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let dev_prefix = format!("/{}", APIStage::Dev.as_str());
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(web::scope(&dev_prefix).configure(scoped_config))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub async fn run_on_lambda() {
    let prod_prefix = format!("/{}", APIStage::Prod.as_str());
    lambda_web::run_actix_on_lambda(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(web::scope(&prod_prefix).configure(scoped_config))
    })
    .await
    .expect("Failed to launch app in lambda runtime")
}
