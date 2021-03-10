#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

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

pub fn create_category(conn: &MysqlConnection, name: &str) -> Category {
    // use schema::category::dsl::name;
    use schema::category::dsl::{category, id};

    let new_cat = NewCategory { name };

    diesel::insert_into(category)
        .values(&new_cat)
        .execute(conn)
        .expect("Error saving new post");

    category.order(id.desc()).first(conn).unwrap()
}

pub fn get_categories(conn: MysqlConnection) -> Vec<Category> {
    use schema::category::dsl::category;
    // use schema::category::dsl::name;

    //let new_cat = NewCategory { name };
    let results = category
        .load::<Category>(&conn)
        .expect("Error loading posts");
    results
}

pub fn create_user(conn: &MysqlConnection, eemail: &str, password: &str) -> User {
    // use schema::category::dsl::name;

    let email = String::from(eemail);
    let password = String::from(password);

    let new_user = User { email, password };

    //use schema::user::dsl::{email, user};
    use schema::user::dsl::user;

    diesel::insert_into(user)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    let email = String::from(eemail);
    let password = String::from("");

    let rtn = User { email, password };
    rtn
    //.get_results(conn);

    //user.order(email).first(conn).unwrap()

    //user.first(conn).unwrap();
    // user.filter(email.eq(&email))
    //     //.limit(5)
    //     .load::<User>(conn)
    //     .expect("Error loading posts");
}
