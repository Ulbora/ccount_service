use ccount_service::create_user;
use ccount_service::establish_connection;

mod tests {
    use super::*;

    #[test]
    fn new_user() {
        let ncon = establish_connection();
        let user = create_user(&ncon, "tester@test.com", "12345");
        assert!(user.email == "tester@test.com");
    }
}
