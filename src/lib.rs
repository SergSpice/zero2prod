use std::net::TcpListener;

use actix_web::{HttpResponse, HttpServer, App, web};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct Formdata {
    email: String,
    name: String
}

async fn health_check() ->  HttpResponse {
    return HttpResponse::Ok().finish();
}

async fn subscribe(_form: web::Form<Formdata>) ->  HttpResponse {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    
    return Ok(server);
}
