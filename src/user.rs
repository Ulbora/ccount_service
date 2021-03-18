use crate::database::user_db::create_user;
use crate::database::user_db::get_user;
use crate::database::user_db::update_user;
use crate::MysqlConnection;
use crate::User;

extern crate argon2;
use argon2::Config;

//use ccount_service::user_db::create_user;
pub fn add_new_user(conn: &MysqlConnection, eemail: &str, password: &str) -> bool {
    let mut suc = false;
    let npw = &hash_password_arg(password, eemail);
    match npw {
        Ok(h) => {
            suc = create_user(conn, eemail, h);
        }
        Err(_) => {}
    }
    suc
}

pub fn login_user(conn: &MysqlConnection, eemail: &str, password: &str) -> bool {
    let us = get_user(conn, eemail);
    let rtn = validate_password_arg(password, &us.password);
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
    let opw_valid = validate_password_arg(password, &us.password);
    if opw_valid {
        let npw = &hash_password_arg(new_password, eemail).unwrap();
        update_user(conn, eemail, npw);
        let us = get_user(conn, eemail);
        rtn = validate_password_arg(new_password, &us.password);
    }
    rtn
}

pub fn hash_password_arg(pw: &str, salt: &str) -> Result<String, argon2::Error> {
    let config = Config::default();
    let h = argon2::hash_encoded(pw.as_bytes(), salt.as_bytes(), &config);
    h
}

pub fn validate_password_arg(pw: &str, hashpw: &str) -> bool {
    let mut rtn = false;
    let rtnm = argon2::verify_encoded(hashpw, pw.as_bytes());
    match rtnm {
        Ok(v) => {
            rtn = v;
        }
        Err(_) => {}
    }
    rtn
}

#[cfg(test)]
mod tests {
    use crate::user::hash_password_arg;
    use crate::user::validate_password_arg;

    #[test]
    fn hash_pw_arg() {
        let hp = hash_password_arg("test", "saltddddd");
        assert!(hp.unwrap() != "");
    }

    #[test]
    fn hash_pw_arg_fail() {
        let mut suc = false;
        let hp = hash_password_arg("test", "salt");
        match hp {
            Ok(v) => {
                if v != "" {
                    suc = true;
                }
            }
            Err(_) => {}
        }
        assert!(!suc);
    }

    #[test]
    fn validate_pw_arg() {
        let hp = hash_password_arg("test", "saltddddd");
        let vd = validate_password_arg("test", &hp.unwrap());
        assert!(vd == true);
    }
}
