// Web microservice for four basic math operations: +, -, *, /

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the best Calculator microservice")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/substract/{a}/{b}")]
async fn substract(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::substract(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::divide(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(substract)
            .service(multiply)
            .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
