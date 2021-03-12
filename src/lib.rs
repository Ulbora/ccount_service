#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod category_db;
pub mod daily_calory_db;
pub mod food_db;
pub mod models;
pub mod schema;
pub mod user_db;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Category, DailyCalories, Food, NewCategory, NewDailyCalories, NewFood, User};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").unwrap_or("mysql://admin:admin@localhost:3306/ccount".to_string());
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
