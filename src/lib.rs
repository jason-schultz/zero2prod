use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");

//     format!("Hello {}!", &name)
// }

// can remove _req: HttpRequest as we don't need it for this endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

// We need to makr 'run' as public
// it is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
