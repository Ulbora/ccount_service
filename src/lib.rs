#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod categorydb;
pub mod models;
pub mod schema;
pub mod userdb;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Category, DailyCalories, Food, NewCategory, User};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
