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

pub fn create_user(conn: &MysqlConnection, eemail: &str, password: &str) -> bool {
    let mut rtn = false;
    let email = String::from(eemail);
    let password = String::from(password);

    let new_user = User { email, password };

    //use schema::user::dsl::{email, user};

    let cnt = diesel::insert_into(user).values(&new_user).execute(conn);
    //.expect("Error saving new user");

    let email = String::from(eemail);
    let password = String::from("");

    //let rtn = User { email, password };
    match cnt {
        Ok(_) => rtn = true,
        Err(_) => rtn = false,
    }
    //assert_eq!(1, cnt);
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
    let data = User {
        email: (&"").to_string(),
        password: "".to_string(),
    };
    let datam = user.filter(email.eq(eemail)).first(conn);
    match datam {
        Ok(u) => {
            return u;
        }
        Err(_) => {
            return data;
        }
    }
}

pub fn delete_user(conn: &MysqlConnection, eemail: &str) -> Result<usize, Box<dyn Error>> {
    use crate::diesel::query_dsl::filter_dsl::FilterDsl;
    use schema::user::dsl::email;
    let num_deleted = diesel::delete(user.filter(email.eq(eemail)))
        .execute(conn)
        .expect("Error deleting user");

    //println!("Deleted {} posts", num_deleted);
    Ok(num_deleted)
}
