use ccount_service::create_category;
//use ccount_service::create_user;
use ccount_service::establish_connection;
use ccount_service::get_categories;

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cat() {
        let ncon = establish_connection();
        let ncat = create_category(&ncon, "snacks");
        assert_ne!(ncat.id, 0);
    }

    #[test]
    fn get_cat_list() {
        let ncon = establish_connection();
        let cats = get_categories(ncon);
        assert!(cats.len() >= 0);
    }

    // #[test]
    // fn new_user() {
    //     let ncon = establish_connection();
    //     let user = create_user(&ncon, "tester@test.com", "12345");
    //     assert!(user.email == "tester@test.com");
    // }
}
