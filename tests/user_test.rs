//use ccount_service::establish_connection;
use ccount_service::establish_pooled_connection;
use ccount_service::user::add_new_user;
use ccount_service::user::change_password;
use ccount_service::user::login_user;
use ccount_service::user_db::delete_user;

mod tests {
    use super::*;

    #[test]
    fn new_user() {
        //let ncon = establish_connection();
        //&pool.get().unwrap()
        let ncon = establish_pooled_connection();
        let user = add_new_user(&ncon.get().unwrap(), "tester2@test.com", "ddffghjk");
        assert!(user.email == "tester2@test.com");

        let vd = login_user(&ncon.get().unwrap(), "tester2@test.com", "ddffghjk");
        assert!(vd == true);

        let vd2 = change_password(
            &ncon.get().unwrap(),
            "tester2@test.com",
            "ddffghjk",
            "12345ggggg",
        );
        assert!(vd2 == true);

        // let user = update_user(&ncon, "tester@test.com", "newpw");
        // assert!(user.email == "tester@test.com");

        //let user = get_user(&ncon, "tester2@test.com");

        //assert!(user.password == "newpw");

        let cnt = delete_user(&ncon.get().unwrap(), "tester2@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
