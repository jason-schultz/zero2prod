use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// can remove _req: HttpRequest as we don't need it for this endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Notice the different signature
// We return `Server` on the happy path and we dropped the `async` keywrod
// We have no .await call, so it is not needed anymore
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // .bind(address)?
    .listen(listener)?
    .run();

    // no .await here
    Ok(server)
}
