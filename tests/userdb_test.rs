use ccount_service::database::user_db::create_user;
use ccount_service::database::user_db::delete_user;
use ccount_service::database::user_db::get_user;
use ccount_service::database::user_db::update_user;
use ccount_service::establish_connection;

mod tests {
    use super::*;

    #[test]
    fn new_user() {
        let ncon = establish_connection();
        let suc = create_user(&ncon, "tester@test.com", "12345");
        assert!(suc == true);

        let user = update_user(&ncon, "tester@test.com", "newpw");
        assert!(user.email == "tester@test.com");

        let user = get_user(&ncon, "tester@test.com");
        assert!(user.password == "newpw");

        let cnt = delete_user(&ncon, "tester@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
