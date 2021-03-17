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

// pub fn hash_password(pw: &str) -> Result<String, Error> {
//     let h = bcrypt::hash(pw);
//     h
// }

// pub fn validate_password(pw: &str, hashpw: &str) -> bool {
//     let rtn = unix::verify(pw, hashpw);
//     rtn
// }

// pub fn hash_password_sha1(pw: &str) -> Result<String, Error> {
//     let hh = "$sha1$19703$iVdJqfSE$v4qYKl1zqYThwpjJAoKX6UvlHq/a";
//     let h = sha1_crypt::hash_with(hh, pw);
//     h
// }

// pub fn validate_password_sha1(pw: &str, hashpw: &str) -> bool {
//     let rtn = sha1_crypt::verify(pw, hashpw);
//     rtn
// }

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
    // #[test]

    // fn hash_pw() {
    //     let hp = hash_password("12456hhhhh");
    //     assert!(hp.unwrap() != "");
    // }

    // #[test]

    // fn hash_pw_fail() {
    //     let hp = hash_password("");
    //     assert!(hp.unwrap() != "");
    // }

    // #[test]
    // fn hash_pw_sha1() {
    //     let hp = hash_password_sha1("test");
    //     assert!(hp.unwrap() != "");
    // }

    // #[test]
    // fn hash_pw_sha1_fail() {
    //     let hp = hash_password_sha1("");
    //     assert!(hp.unwrap() != "");
    // }

    // #[test]
    // fn validate_pw() {
    //     let hp = hash_password("12456hhhhh");
    //     let vd = validate_password("12456hhhhh", &hp.unwrap());
    //     assert!(vd == true);
    // }

    // #[test]
    // fn validate_pw_sha1() {
    //     let hp = hash_password_sha1("12456hhhhh");
    //     let vd = validate_password_sha1("12456hhhhh", &hp.unwrap());
    //     assert!(vd == true);
    // }

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
