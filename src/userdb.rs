use crate::diesel;
use std::error::Error;

use crate::diesel::query_dsl::methods::OrderDsl;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::schema;
use crate::schema::user::dsl::user;
//use crate::schema::user::email;
use crate::MysqlConnection;
use crate::User;

pub fn create_user(conn: &MysqlConnection, eemail: &str, password: &str) -> User {
    let email = String::from(eemail);
    let password = String::from(password);

    let new_user = User { email, password };

    //use schema::user::dsl::{email, user};

    diesel::insert_into(user)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post");

    let email = String::from(eemail);
    let password = String::from("");

    let rtn = User { email, password };
    rtn
}

pub fn update_user(conn: &MysqlConnection, eemail: &str, pw: &str) -> User {
    use crate::diesel::query_dsl::filter_dsl::FindDsl;
    use schema::user::dsl::password;

    diesel::update(user.find(eemail))
        .set(password.eq(pw))
        .execute(conn)
        .unwrap();

    let email = String::from(eemail);
    let passw = String::from("");

    let rtn = User {
        email,
        password: passw,
    };
    rtn
}

pub fn get_user(conn: &MysqlConnection, eemail: &str) -> User {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use schema::user::dsl::email;
    let data = user.filter(email.eq(eemail)).first(conn).unwrap();
    data
}

pub fn delete_user(conn: &MysqlConnection, eemail: &str) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use schema::user::dsl::email;
    let num_deleted = diesel::delete(user.filter(email.eq(eemail)))
        .execute(conn)
        .expect("Error deleting posts");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
