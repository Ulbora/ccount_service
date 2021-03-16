use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use ccount_service::user::add_new_user;
use ccount_service::user::login_user;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::MysqlConnection;

use serde::Deserialize;
use serde::Serialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello calorie counting world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
struct User {
    email: String,
    password: String,
    // new_password: String,
}

#[derive(Deserialize)]
struct UserPw {
    email: String,
    password: String,
    new_password: String,
}

#[derive(Serialize)]
struct Resp {
    success: bool,
}

#[post("/user/new")]
async fn new_user(
    req_body: web::Json<User>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let auth = req.headers().get("authorization");
    println!("authorization: {:?}", auth);
    let suc = add_new_user(&pool.get().unwrap(), &req_body.email, &req_body.password);

    let res = Resp { success: suc };
    if suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else {
        HttpResponse::Conflict()
            .content_type("application/json")
            .json(res)
    }
}

#[post("/user/login")]
async fn login(
    req_body: web::Json<User>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> impl Responder {
    let suc = login_user(&pool.get().unwrap(), &req_body.email, &req_body.password);

    let res = Resp { success: suc };
    if suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else {
        HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(res)
    }
}
