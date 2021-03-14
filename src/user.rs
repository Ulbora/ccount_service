use crate::database::user_db::create_user;
use crate::database::user_db::get_user;
use crate::database::user_db::update_user;
use crate::MysqlConnection;
use crate::User;
use pwhash::bcrypt;
use pwhash::error::Error;
use pwhash::unix;

//use ccount_service::user_db::create_user;
pub fn add_new_user(conn: &MysqlConnection, eemail: &str, password: &str) -> User {
    let npw = &hash_password(password).unwrap();
    let user = create_user(conn, eemail, npw);
    user
}

pub fn login_user(conn: &MysqlConnection, eemail: &str, password: &str) -> bool {
    let us = get_user(conn, eemail);
    let rtn = validate_password(password, &us.password);
    rtn
}

pub fn change_password(
    conn: &MysqlConnection,
    eemail: &str,
    password: &str,
    new_password: &str,
) -> bool {
    let mut rtn = false;
    let us = get_user(conn, eemail);
    let opw_valid = validate_password(password, &us.password);
    if opw_valid {
        let npw = &hash_password(new_password).unwrap();
        update_user(conn, eemail, npw);
        let us = get_user(conn, eemail);
        rtn = validate_password(new_password, &us.password);
    }
    rtn
}

pub fn hash_password(pw: &str) -> Result<String, Error> {
    let h = bcrypt::hash(pw);
    h
}

pub fn validate_password(pw: &str, hashpw: &str) -> bool {
    let rtn = unix::verify(pw, hashpw);
    rtn
}

#[cfg(test)]
mod tests {
    use crate::user::hash_password;
    use crate::user::validate_password;
    #[test]

    fn hash_pw() {
        let hp = hash_password("12456hhhhh");
        assert!(hp.unwrap() != "");
    }

    #[test]

    fn hash_pw_fail() {
        let hp = hash_password("");
        assert!(hp.unwrap() != "");
    }

    #[test]
    fn validate_pw() {
        let hp = hash_password("12456hhhhh");
        let vd = validate_password("12456hhhhh", &hp.unwrap());
        assert!(vd == true);
    }
}
