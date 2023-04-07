use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize, Serialize)]
struct Subscriber {
    email: String,
    name: String,
}

async fn subscribe(_: web::Form<Subscriber>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
