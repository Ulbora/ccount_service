use crate::MysqlConnection;
use std::error::Error;

use crate::DailyCalories;

use crate::database::daily_calory_db::create_daily_calories;
use crate::database::daily_calory_db::delete_daily_calories;
use crate::database::daily_calory_db::get_daily_calories;

use crate::database::food_db::get_food_by_id;

pub fn create_new_daily_calories(
    conn: &MysqlConnection,
    day: &str,
    uemail: &str,
    fid: i64,
) -> DailyCalories {
    let rtn = create_daily_calories(conn, day, uemail, fid);
    rtn
}

pub fn get_calories_for_day(conn: &MysqlConnection, uemail: &str, dday: &str) -> i32 {
    let mut rtn = 0;
    let dcl = get_daily_calories(conn, uemail, dday);
    for dc in dcl.iter() {
        let fd = get_food_by_id(conn, dc.food_id);
        rtn += fd.calories;
    }
    rtn
}

pub fn delete_existing_daily_calory(
    conn: &MysqlConnection,
    dcid: i64,
) -> Result<usize, Box<dyn Error>> {
    let rtn = delete_daily_calories(conn, dcid);
    rtn
}
