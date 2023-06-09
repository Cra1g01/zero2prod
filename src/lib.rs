#![allow(clippy::needless_return)]

use actix_web::{web, App, HttpServer, HttpResponse, dev::Server};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    return Ok(server);
}

