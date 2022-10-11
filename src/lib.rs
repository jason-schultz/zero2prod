use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

// can remove _req: HttpRequest as we don't need it for this endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// Notice the different signature
// We return `Server` on the happy path and we dropped the `async` keywrod
// We have no .await call, so it is not needed anymore
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        // .bind(address)?
        .listen(listener)?
        .run();

    // no .await here
    Ok(server)
}
