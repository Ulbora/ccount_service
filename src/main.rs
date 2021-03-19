extern crate actix_cors;
use crate::routes::change_pw;
use crate::routes::delete_calories;
use crate::routes::delete_food;

use crate::routes::get_calories_by_day;
use crate::routes::get_calory_list_by_day;
use crate::routes::get_cat_list;
use crate::routes::get_food_list_by_cat;

use crate::routes::login;

use crate::routes::new_calories;
use crate::routes::new_food;
use crate::routes::new_user;
use crate::routes::update_food;

use actix_web::{http, App, HttpServer};

use actix_cors::Cors;

use ccount_service::establish_pooled_connection;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on port: 8080");
    let pool = establish_pooled_connection();
    HttpServer::new(move || {
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
            .service(new_user)
            .service(login)
            .service(change_pw)
            .service(get_cat_list)
            .service(new_food)
            .service(update_food)
            .service(get_food_list_by_cat)
            .service(delete_food)
            .service(new_calories)
            .service(get_calory_list_by_day)
            .service(get_calories_by_day)
            .service(delete_calories)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
