//use self::models::{Category, DailyCalories, Food, NewCategory, User};

use crate::diesel;
use std::error::Error;

use crate::diesel::query_dsl::methods::OrderDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema;
use crate::schema::category::dsl::category;
use crate::schema::category::id;
use crate::Category;
use crate::MysqlConnection;
use crate::NewCategory;

pub fn create_category(conn: &MysqlConnection, name: &str) -> Category {
    let new_cat = NewCategory { name };

    diesel::insert_into(category)
        .values(&new_cat)
        .execute(conn)
        .expect("Error saving new cat");

    category.order(id.desc()).first(conn).unwrap()
}

pub fn update_category(conn: &MysqlConnection, cid: i64, nm: &str) -> Category {
    use crate::diesel::query_dsl::filter_dsl::FindDsl;

    use schema::category::dsl::name;

    diesel::update(category.find(cid))
        .set(name.eq(nm))
        .execute(conn)
        .unwrap();

    let cname = String::from(nm);

    let rtn = Category {
        id: cid,
        name: cname,
    };
    rtn
}

pub fn get_categories(conn: &MysqlConnection) -> Vec<Category> {
    let results = category.load::<Category>(conn).expect("Error loading cats");
    results
}

pub fn delete_category(conn: &MysqlConnection, cid: i64) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    let num_deleted = diesel::delete(category.filter(id.eq(cid)))
        .execute(conn)
        .expect("Error deleting cat");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
