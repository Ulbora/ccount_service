use crate::category_db::create_category;
use crate::category_db::delete_category;
use crate::category_db::get_categories;
use crate::category_db::update_category;
use std::error::Error;

use crate::MysqlConnection;

use crate::Category;

pub fn add_new_category(conn: &MysqlConnection, name: &str) -> Category {
    let cat = create_category(conn, name);
    cat
}

pub fn update_existing_category(conn: &MysqlConnection, cid: i64, nm: &str) -> Category {
    let cat = update_category(conn, cid, nm);
    cat
}

pub fn get_category_list(conn: &MysqlConnection) -> Vec<Category> {
    let clist = get_categories(conn);
    clist
}

pub fn delete_existing_category(conn: &MysqlConnection, cid: i64) -> Result<usize, Box<dyn Error>> {
    let rtn = delete_category(conn, cid);
    rtn
}
