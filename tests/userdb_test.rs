use ccount_service::establish_connection;
use ccount_service::user_db::create_user;
use ccount_service::user_db::delete_user;
use ccount_service::user_db::get_user;
use ccount_service::user_db::update_user;

mod tests {
    use super::*;

    #[test]
    fn new_user() {
        let ncon = establish_connection();
        let user = create_user(&ncon, "tester@test.com", "12345");
        assert!(user.email == "tester@test.com");

        let user = update_user(&ncon, "tester@test.com", "newpw");
        assert!(user.email == "tester@test.com");

        let user = get_user(&ncon, "tester@test.com");
        assert!(user.password == "newpw");

        let cnt = delete_user(&ncon, "tester@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
