use crate::schema::category;
use crate::schema::daily_calories;
use crate::schema::food;
use crate::schema::user;

#[derive(Queryable, Insertable)]
#[table_name = "category"]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "category"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Insertable)]
#[table_name = "daily_calories"]
pub struct DailyCalories {
    pub id: i64,
    pub day: String,
    pub user_email: String,
    pub food_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "daily_calories"]
pub struct NewDailyCalories<'a> {
    pub day: &'a str,
    pub user_email: &'a str,
    pub food_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "food"]
pub struct Food {
    pub id: i64,
    pub name: String,
    pub calories: i32,
    pub user_email: String,
    pub category_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "food"]
pub struct NewFood<'a> {
    pub name: &'a str,
    pub calories: i32,
    pub user_email: &'a str,
    pub category_id: i64,
}

#[derive(Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub email: String,
    pub password: String,
}
