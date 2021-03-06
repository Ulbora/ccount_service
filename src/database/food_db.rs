use crate::diesel;
use crate::schema::food::user_email;
use std::error::Error;

use crate::diesel::query_dsl::methods::OrderDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema;
use crate::schema::food::dsl::food;
use crate::schema::food::id;
use crate::Food;
use crate::MysqlConnection;
use crate::NewFood;

pub fn create_food(conn: &MysqlConnection, name: &str, cid: i64, cals: i32, uemail: &str) -> Food {
    let new_food = NewFood {
        name,
        category_id: cid,
        calories: cals,
        user_email: uemail,
    };

    diesel::insert_into(food)
        .values(&new_food)
        .execute(conn)
        .expect("Error saving new food");

    food.order(id.desc()).first(conn).unwrap()
}

pub fn update_food(
    conn: &MysqlConnection,
    fid: i64,
    nm: &str,
    cid: i64,
    cals: i32,
    uemail: &str,
) -> Food {
    use crate::diesel::query_dsl::filter_dsl::FindDsl;

    use schema::food::dsl::{calories, name};

    diesel::update(food.find(fid))
        .set((name.eq(nm), calories.eq(cals)))
        .execute(conn)
        .unwrap();

    let fname = String::from(nm);
    let em = String::from(uemail);

    let rtn = Food {
        id: fid,
        name: fname,
        calories: cals,
        category_id: cid,
        user_email: em,
    };
    rtn
}

pub fn get_food_by_category(conn: &MysqlConnection, cid: i64, uemail: &str) -> Vec<Food> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use crate::schema::food::name;
    use schema::food::dsl::category_id;
    let data = food
        .filter(category_id.eq(cid))
        .filter(user_email.eq(uemail))
        .order(name)
        .load::<Food>(conn)
        .expect("Error loading cats");
    data
}

pub fn get_food_by_user(conn: &MysqlConnection, uemail: &str) -> Vec<Food> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use crate::schema::food::name;

    let data = food
        .filter(user_email.eq(uemail))
        .order(name)
        .load::<Food>(conn)
        .expect("Error loading cats");
    data
}

pub fn get_food_by_id(conn: &MysqlConnection, fid: i64) -> Food {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    let data = food.filter(id.eq(fid)).first(conn).unwrap();
    data
}

pub fn delete_food(
    conn: &MysqlConnection,
    fid: i64,
    uemail: &str,
) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    let num_deleted = diesel::delete(food.filter(id.eq(fid)).filter(user_email.eq(uemail)))
        .execute(conn)
        .expect("Error deleting cat");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
