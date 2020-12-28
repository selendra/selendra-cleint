use actix_web::{web, App, HttpServer};

pub mod errors;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get-wallet", web::post().to(services::http_get_wallet))
            .route("/check-balance", web::post().to(services::http_check_balance))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}