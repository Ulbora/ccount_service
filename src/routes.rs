use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use ccount_service::category::get_category_list;
use ccount_service::food::create_new_food;
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
use std::env;

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
    let auth_suc = validate_api_key(req);

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

fn validate_api_key(req: HttpRequest) -> bool {
    let rtn = false;
    let api_key = env::var("api-key").unwrap_or("ddjdt373dcf7dhdh222282727fffeee".to_string());
    let key_h = req.headers().get("api-key");
    match key_h {
        Some(k) => {
            let key = k.to_str().ok().unwrap();
            if key == &api_key {
                return true;
            }
        }
        None => {}
    }
    rtn
}

#[derive(Serialize)]
struct Category {
    pub id: i64,
    pub name: String,
}

#[get("/category/list")]
async fn get_cat_list(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut rtn: Vec<Category> = Vec::new();
    // let lst = get_category_list(&pool.get().unwrap());
    let auth_suc = validate_auth(req, &pool.get().unwrap());
    if auth_suc {
        let lst = get_category_list(&pool.get().unwrap());
        for c in lst {
            let cc = Category {
                id: c.id,
                name: c.name,
            };
            rtn.push(cc);
        }
    }

    // let mut rtn: Vec<Category> = Vec::new(); //= std::vec::Vec<Category>.new();

    if auth_suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(rtn)
    } else {
        HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(rtn)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Food {
    name: String,
    calories: i32,
    user_email: String,
    category_id: i64,
}

#[post("/food/new")]
async fn new_food(
    req_body: web::Json<Food>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_api_key(req);

    println!("authed: {:?}", auth_suc);
    if auth_suc {
        let fd = create_new_food(
            &pool.get().unwrap(),
            &req_body.name,
            req_body.category_id,
            req_body.calories,
            &req_body.user_email,
        );
        if fd.id > 0 {
            suc = true;
        }
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
