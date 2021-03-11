use crate::diesel;
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

pub fn create_food(conn: &MysqlConnection, name: &str, cid: i64, cals: i32) -> Food {
    let new_food = NewFood {
        name,
        category_id: cid,
        calories: cals,
    };

    diesel::insert_into(food)
        .values(&new_food)
        .execute(conn)
        .expect("Error saving new food");

    food.order(id.desc()).first(conn).unwrap()
}

pub fn update_food(conn: &MysqlConnection, fid: i64, nm: &str, cid: i64, cals: i32) -> Food {
    use crate::diesel::query_dsl::filter_dsl::FindDsl;

    use schema::food::dsl::{calories, id, name};

    diesel::update(food.find(fid))
        .set((name.eq(nm), calories.eq(cals)))
        .execute(conn)
        .unwrap();

    let fname = String::from(nm);

    let rtn = Food {
        id: fid,
        name: fname,
        calories: cals,
        category_id: cid,
    };
    rtn
}

pub fn get_food_by_category(conn: &MysqlConnection, fid: i64) -> Vec<Food> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use schema::food::dsl::category_id;
    let data = food
        .filter(category_id.eq(fid))
        .load::<Food>(conn)
        .expect("Error loading cats");
    data
}

pub fn delete_food(conn: &MysqlConnection, fid: i64) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    let num_deleted = diesel::delete(food.filter(id.eq(fid)))
        .execute(conn)
        .expect("Error deleting cat");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
