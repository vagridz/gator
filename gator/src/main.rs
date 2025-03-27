use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

async fn docs() -> impl Responder {
    "docs"
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .route("/healthcheck", web::get().to(health_check))
            .service(web::scope("/internals").route("/docs", web::get().to(docs)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
