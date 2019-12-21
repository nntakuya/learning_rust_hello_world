use actix_web::{web, App, HttpResponse, HttpServer,  Responder};

fn index() -> impl Responder {
    "Hello world!"
}

#[rustfmt::skip]
pub fn main() {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/index.html", web::to(|| HttpResponse::Ok())))
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| HttpResponse::Ok())))
    })
    .bind("128.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}


