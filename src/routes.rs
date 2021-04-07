use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use ccount_service::category::get_category_list;
use ccount_service::daily_calories::create_new_daily_calories;
use ccount_service::daily_calories::delete_existing_daily_calory;
use ccount_service::daily_calories::get_calories_for_day;
use ccount_service::daily_calories::get_calories_list_for_day;
use ccount_service::food::create_new_food;
use ccount_service::food::delete_existing_food;
use ccount_service::food::get_food_list_by_category;
use ccount_service::food::get_food_list_by_user;
use ccount_service::food::update_existing_food;
use ccount_service::user::add_new_user;
use ccount_service::user::change_password;
use ccount_service::user::login_user;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::MysqlConnection;

use serde::Deserialize;
use serde::Serialize;
extern crate base64;

use std::env;

#[derive(Deserialize)]
struct User {
    email: String,
    password: String,
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

#[derive(Serialize)]
struct CountResp {
    calories: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewDailyCalories {
    day: String,
    user_email: String,
    food_id: i64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DailyCalories {
    id: i64,
    day: String,
    user_email: String,
    food_id: i64,
}

#[derive(Serialize)]
struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Food {
    id: i64,
    name: String,
    calories: i32,
    user_email: String,
    category_id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewFood {
    name: String,
    calories: i32,
    user_email: String,
    category_id: i64,
}

#[post("/user/new")]
async fn new_user(
    req_body: web::Json<User>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_api_key(req);

    //println!("authed: {:?}", auth_suc);
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
    let auth_suc = validate_auth(req, &req_body.email, &pool.get().unwrap());
    //println!("authed: {:?}", auth_suc);
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

fn validate_auth(req: HttpRequest, email: &str, conn: &MysqlConnection) -> bool {
    let mut rtn = false;
    let auth = req.headers().get("authorization");
    match auth {
        Some(a) => {
            let creds = a.to_str().ok().unwrap();
            //println!("creds: {:?}", creds);
            let decred = &base64::decode(creds);
            match decred {
                Ok(c) => {
                    let bytes = &c[..];
                    let astr = std::str::from_utf8(bytes).unwrap();
                    let creds: Vec<&str> = astr.split(':').collect();
                    if creds.len() == 2 {
                        //println!("email is {}", creds[0]);
                        //println!("pw is {}", creds[1]);
                        if email == creds[0] {
                            rtn = login_user(conn, creds[0], creds[1]);
                        }
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
    let api_key = env::var("API_KEY").unwrap_or("ddjdt373dcf7dhdh222282727fffeee".to_string());
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

#[get("/category/list")]
async fn get_cat_list(pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>) -> impl Responder {
    let mut rtn: Vec<Category> = Vec::new();
    let lst = get_category_list(&pool.get().unwrap());
    for c in lst {
        let cc = Category {
            id: c.id,
            name: c.name,
        };
        rtn.push(cc);
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .json(rtn)
}

#[post("/food/new")]
async fn new_food(
    req_body: web::Json<NewFood>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &req_body.user_email, &pool.get().unwrap());

    //println!("authed: {:?}", auth_suc);
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

#[put("/food/update")]
async fn update_food(
    req_body: web::Json<Food>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &req_body.user_email, &pool.get().unwrap());

    //println!("authed: {:?}", auth_suc);
    if auth_suc {
        let fd = update_existing_food(
            &pool.get().unwrap(),
            req_body.id,
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

#[get("/food/list/{cid}/{email}")]
async fn get_food_list_by_cat(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path((cid, email)): web::Path<(i64, String)>,
) -> impl Responder {
    let mut rtn: Vec<Food> = Vec::new();
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    if auth_suc {
        let lst = get_food_list_by_category(&pool.get().unwrap(), cid, &email);
        for f in lst {
            let ff = Food {
                id: f.id,
                name: f.name,
                category_id: f.category_id,
                calories: f.calories,
                user_email: f.user_email,
            };
            rtn.push(ff);
        }
    }
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

#[get("/food/list/{email}")]
async fn get_food_list_by_users(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path(email): web::Path<String>,
) -> impl Responder {
    let mut rtn: Vec<Food> = Vec::new();
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    if auth_suc {
        let lst = get_food_list_by_user(&pool.get().unwrap(), &email);
        for f in lst {
            let ff = Food {
                id: f.id,
                name: f.name,
                category_id: f.category_id,
                calories: f.calories,
                user_email: f.user_email,
            };
            rtn.push(ff);
        }
    }
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

#[delete("/food/{id}/{email}")]
async fn delete_food(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path((id, email)): web::Path<(i64, String)>,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    if auth_suc {
        let dsuc = delete_existing_food(&pool.get().unwrap(), id, &email);
        match dsuc {
            Ok(d) => {
                if d == 1 {
                    suc = true;
                }
            }
            Err(_) => {}
        }
    }
    let res = Resp { success: suc };
    if auth_suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else {
        HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(res)
    }
}

#[post("/calories/new")]
async fn new_calories(
    req_body: web::Json<NewDailyCalories>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &req_body.user_email, &pool.get().unwrap());

    //println!("authed: {:?}", auth_suc);
    if auth_suc {
        let fd = create_new_daily_calories(
            &pool.get().unwrap(),
            &req_body.day,
            &req_body.user_email,
            req_body.food_id,
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

#[get("/calory/list/{email}/{day}")]
async fn get_calory_list_by_day(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path((email, day)): web::Path<(String, String)>,
) -> impl Responder {
    let mut rtn: Vec<DailyCalories> = Vec::new();
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    if auth_suc {
        let lst = get_calories_list_for_day(&pool.get().unwrap(), &email, &day);
        for c in lst {
            let dc = DailyCalories {
                id: c.id,
                day: c.day,
                user_email: c.user_email,
                food_id: c.food_id,
            };
            rtn.push(dc);
        }
    }
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

#[get("/calories/{email}/{day}")]
async fn get_calories_by_day(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path((email, day)): web::Path<(String, String)>,
) -> impl Responder {
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    let mut cnt = 0;
    if auth_suc {
        cnt = get_calories_for_day(&pool.get().unwrap(), &email, &day);
    }
    let rtn = CountResp { calories: cnt };
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

#[delete("/calories/{id}/{email}")]
async fn delete_calories(
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
    req: HttpRequest,
    web::Path((id, email)): web::Path<(i64, String)>,
) -> impl Responder {
    let mut suc = false;
    let auth_suc = validate_auth(req, &email, &pool.get().unwrap());
    if auth_suc {
        let dsuc = delete_existing_daily_calory(&pool.get().unwrap(), id, &email);
        match dsuc {
            Ok(d) => {
                if d == 1 {
                    suc = true;
                }
            }
            Err(_) => {}
        }
    }
    let res = Resp { success: suc };
    if auth_suc {
        HttpResponse::Ok()
            .content_type("application/json")
            .json(res)
    } else {
        HttpResponse::Unauthorized()
            .content_type("application/json")
            .json(res)
    }
}
