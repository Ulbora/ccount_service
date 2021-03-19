use crate::diesel;
use crate::schema::daily_calories::user_email;
use std::error::Error;

use crate::diesel::query_dsl::methods::OrderDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema;
use crate::schema::daily_calories::dsl::daily_calories;
use crate::schema::daily_calories::id;
use crate::DailyCalories;
use crate::MysqlConnection;
use crate::NewDailyCalories;

pub fn create_daily_calories(
    conn: &MysqlConnection,
    day: &str,
    uemail: &str,
    fid: i64,
) -> DailyCalories {
    let new_daily_cal = NewDailyCalories {
        day,
        user_email: uemail,
        food_id: fid,
    };

    diesel::insert_into(daily_calories)
        .values(&new_daily_cal)
        .execute(conn)
        .expect("Error saving new food");

    daily_calories.order(id.desc()).first(conn).unwrap()
}

pub fn get_daily_calories(conn: &MysqlConnection, uemail: &str, dday: &str) -> Vec<DailyCalories> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use schema::daily_calories::dsl::day;
    let data = daily_calories
        //.filter(user_email.eq(uemail).and(day.eq(dday)))
        .filter(user_email.eq(uemail))
        .filter(day.eq(dday))
        .load::<DailyCalories>(conn)
        .expect("Error loading cats");
    data
}

pub fn delete_daily_calories(
    conn: &MysqlConnection,
    dcid: i64,
    uemail: &str,
) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    let num_deleted = diesel::delete(
        daily_calories
            .filter(id.eq(dcid))
            .filter(user_email.eq(uemail)),
    )
    .execute(conn)
    .expect("Error deleting cat");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
