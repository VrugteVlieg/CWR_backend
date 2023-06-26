use crate::models::scoped::*;
use actix_web::{get, web, post, HttpResponse};

#[get("/")]
async fn scoped_root(data: web::Data<ScopedState>) -> String {
    let scope_name = &data.scope_name;
    format!("Hello from {scope_name}")
}

#[post("/")]
async fn scoped_root_post(animal: web::Json<Animal>, state: web::Data<ScopedState>) -> String {
    format!("Hello postie \nThe current app state is {state:?}\nYou provided the animal {:?}", animal)
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .app_data(web::Data::new(ScopedState {
        scope_name: String::from("scope")
    }))
    .service(scoped_root)
    .service(scoped_root_post)
    .service(
        web::resource("/scope")
        .route(web::get().to(|| async { HttpResponse::Ok().body("scope")}))
        .route(web::post().to(HttpResponse::MethodNotAllowed))
    );
}