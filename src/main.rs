extern crate actix_cors;
use crate::routes::change_pw;
use crate::routes::echo;
use crate::routes::get_cat_list;
use crate::routes::get_food_list_by_cat;
use crate::routes::hello;
use crate::routes::login;
use crate::routes::manual_hello;
use crate::routes::new_food;
use crate::routes::new_user;
use crate::routes::update_food;
//use actix_web::dev::ServiceRequest;
//use actix_web::service::ServiceRequest;
use actix_web::Error;
use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};

use actix_cors::Cors;

use ccount_service::establish_pooled_connection;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on port: 8080");
    let pool = establish_pooled_connection();
    HttpServer::new(move || {
        //let auth = HttpAuthentication::basic(basic_auth_validator);
        App::new()
            .wrap(
                Cors::default()
                    //.allowed_origin("http://127.0.0.1:3000")
                    //.allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .allowed_header("api-key")
                    .max_age(3600),
            )
            .service(hello)
            .service(echo)
            .service(new_user)
            .service(login)
            .service(change_pw)
            .service(get_cat_list)
            .service(new_food)
            .service(update_food)
            .service(get_food_list_by_cat)
            .data(pool.clone())
            // .route("/user/new", web::get().to(new_user))
            //.service(web::resource("/user/new").route(web::post().to_async(new_user)))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
