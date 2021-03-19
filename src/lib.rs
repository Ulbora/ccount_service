#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod category;
pub mod daily_calories;
pub mod database;
pub mod food;
pub mod models;
pub mod schema;
pub mod user;

use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

use self::models::{Category, DailyCalories, Food, NewCategory, NewDailyCalories, NewFood, User};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").unwrap_or("mysql://admin:admin@localhost:3306/ccount".to_string());
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_pooled_connection() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url =
        env::var("DATABASE_URL").unwrap_or("mysql://admin:admin@localhost:3306/ccount".to_string());

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create postgres pool.");
    pool
}
