use actix_web::HttpResponse;

// can remove _req: HttpRequest as we don't need it for this endpoint
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
