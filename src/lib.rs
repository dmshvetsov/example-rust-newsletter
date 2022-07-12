use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    println!("starting http server on http://127.0.0.1:{}", port);

    Ok(server)
}
