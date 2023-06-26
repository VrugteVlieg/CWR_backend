mod endpoints;
mod models;

use actix_web::{web, App, HttpServer};
use endpoints::{root, scoped};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .configure(root::config)
        .service(
            web::scope("/animal")
            .configure(scoped::config)
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}