use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check, subscribe};

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new().wrap(TracingLogger::default()).service(
            web::scope("/api")
                .service(health_check)
                .service(web::scope("/v1").service(subscribe)),
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
