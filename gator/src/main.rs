use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn docs(data: web::Data<AppState>, shared_data: web::Data<AppStateWithCounter>) -> String {
    let app_name = &data.app_name;
    let mut counter = shared_data.counter.lock().unwrap();
    *counter += 1;
    format!("Number of requests for the docs of {app_name}: {counter}")
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .service(home)
            .route("/healthcheck", web::get().to(health_check))
            .service(web::scope("/internals").route("/docs", web::get().to(docs)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
