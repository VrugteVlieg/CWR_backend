use crate::models::global_models::AppState;

use actix_web::{get, web};

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[get("/counter")]
async fn increment_counter(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix Web"),
        counter: std::sync::Mutex::new(0),
    });

    cfg.app_data(app_state.clone())
        .service(index)
        .service(increment_counter);
}
