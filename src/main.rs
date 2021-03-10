use crate::routes::echo;
use crate::routes::hello;
use crate::routes::manual_hello;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on port: 8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
