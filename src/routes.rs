use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use ccount_service::user::add_new_user;
use ccount_service::user::change_password;
use ccount_service::user::login_user;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::MysqlConnection;

use serde::Deserialize;
use serde::Serialize;
extern crate base64;

use base64::decode;

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
#[serde(rename_all = "camelCase")]
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
    let mut suc = false;
    let auth_suc = validate_auth(req, &pool.get().unwrap());
    println!("authed: {:?}", auth_suc);
    if auth_suc {
        suc = add_new_user(&pool.get().unwrap(), &req_body.email, &req_body.password);
    }
    let res = Resp { success: suc };
    if suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else if !auth_suc {
        HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(res)
    } else {
        HttpResponse::Conflict()
            .content_type("application/json")
            .json(res)
    }
}

#[post("/user/change/pw")]
async fn change_pw(
    req_body: web::Json<UserPw>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &pool.get().unwrap());
    println!("authed: {:?}", auth_suc);
    if auth_suc {
        suc = change_password(
            &pool.get().unwrap(),
            &req_body.email,
            &req_body.password,
            &req_body.new_password,
        );
    }
    let res = Resp { success: suc };
    if suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else if !auth_suc {
        HttpResponse::Unauthorized()
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

fn validate_auth(req: HttpRequest, conn: &MysqlConnection) -> bool {
    let mut rtn = false;
    let auth = req.headers().get("authorization");
    match auth {
        Some(a) => {
            let creds = a.to_str().ok().unwrap();
            //println!("creds: {:?}", creds);
            let decred = &base64::decode(creds);
            match decred {
                Ok(c) => {
                    // let bytes = &base64::decode(creds).unwrap()[..];
                    let bytes = &c[..];
                    let astr = std::str::from_utf8(bytes).unwrap();
                    let creds: Vec<&str> = astr.split(':').collect();
                    if creds.len() == 2 {
                        //println!("email is {}", creds[0]);
                        //println!("pw is {}", creds[1]);
                        rtn = login_user(conn, creds[0], creds[1]);
                    }
                }
                Err(_) => {}
            }
        }
        None => {}
    }
    rtn
}
