use crate::diesel;
use crate::schema::daily_calories::user_email;
use diesel::sql_types::Integer;
use diesel::sql_types::Text;

use diesel::types::Varchar;

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

pub fn get_day_list(conn: &MysqlConnection, uemail: &str, days: i64) -> Vec<String> {
    use crate::diesel::QueryDsl;
    use schema::daily_calories::dsl::day;
    let data = diesel::query_dsl::methods::OrderDsl::order(
        diesel::query_dsl::methods::OrderDsl::order(
            diesel::query_dsl::filter_dsl::FilterDsl::filter(daily_calories, user_email.eq(uemail)),
            day,
        )
        .select(day)
        .limit(days)
        .distinct(),
        day.desc(),
    )
    .load::<String>(conn)
    .expect("Error loading cats");
    data
}

pub fn get_daily_calories(conn: &MysqlConnection, uemail: &str, dday: &str) -> Vec<DailyCalories> {
    use crate::diesel::QueryDsl;
    use crate::schema::daily_calories::food_id;
    //use crate::schema::daily_calories::id;
    use crate::schema::food::dsl::food;
    use schema::daily_calories::dsl::day;
    use schema::food::dsl::name;
    let data = diesel::query_dsl::methods::OrderDsl::order(
        diesel::query_dsl::filter_dsl::FilterDsl::filter(
            diesel::query_dsl::filter_dsl::FilterDsl::filter(
                daily_calories
                    .inner_join(food)
                    .select((id, day, user_email, food_id)),
                user_email.eq(uemail),
            ),
            day.eq(dday),
        ),
        name,
    )
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

#[derive(QueryableByName, Debug)]
pub struct CaloryCount {
    #[sql_type = "Varchar"]
    pub total: String,
    #[sql_type = "Varchar"]
    pub day: String,
}

pub fn calories_for_multi_days(
    conn: &MysqlConnection,
    uemail: &str,
    days: i32,
) -> Vec<CaloryCount> {
    //println!("get_calories_for_multi_days");

    let data = diesel::sql_query(
        "select sum(f.calories) as total, dc.day from food f inner join daily_calories dc  on dc.food_id = f.id where dc.user_email = ? group by dc.day order by dc.day desc limit ? ")
        .bind::<Text, _>(uemail)
        .bind::<Integer, _>(days)
        .load::<CaloryCount>(conn).expect("Error loading cats");
    // println!("q res: {:?}", &data);
    data
}

// select sum(f.calories), dc.day
// from food f
// inner join daily_calories dc
// on dc.food_id = f.id
// where dc.user_email = 'ken21@ken.com'
// group by dc.day
// order by dc.day
