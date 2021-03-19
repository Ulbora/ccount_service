use crate::database::food_db::create_food;
use crate::database::food_db::delete_food;
use crate::database::food_db::get_food_by_category;
use crate::database::food_db::update_food;
use std::error::Error;

use crate::MysqlConnection;

use crate::Food;

pub fn create_new_food(
    conn: &MysqlConnection,
    name: &str,
    cid: i64,
    cals: i32,
    uemail: &str,
) -> Food {
    let fd = create_food(conn, name, cid, cals, uemail);
    fd
}

pub fn update_existing_food(
    conn: &MysqlConnection,
    fid: i64,
    name: &str,
    cid: i64,
    cals: i32,
    uemail: &str,
) -> Food {
    let fd = update_food(conn, fid, name, cid, cals, uemail);
    fd
}

pub fn get_food_list_by_category(conn: &MysqlConnection, cid: i64, uemail: &str) -> Vec<Food> {
    let flst = get_food_by_category(conn, cid, uemail);
    flst
}

pub fn delete_existing_food(
    conn: &MysqlConnection,
    fid: i64,
    uemail: &str,
) -> Result<usize, Box<dyn Error>> {
    let rtn = delete_food(conn, fid, uemail);
    rtn
}
