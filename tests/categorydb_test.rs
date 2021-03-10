use ccount_service::categorydb::create_category;
//use ccount_service::create_user;
use ccount_service::categorydb::delete_category;
use ccount_service::categorydb::get_categories;
use ccount_service::categorydb::update_category;
use ccount_service::establish_connection;

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cat() {
        let ncon = establish_connection();
        let ncat = create_category(&ncon, "snacks");
        assert_ne!(ncat.id, 0);

        let ocat = update_category(&ncon, ncat.id, "drinks");
        assert_eq!(ocat.name, "drinks");

        let cats = get_categories(&ncon);
        assert!(cats.len() >= 1);

        let cnt = delete_category(&ncon, ncat.id);
        assert!(cnt.unwrap() == 1);
    }

    // #[test]
    // fn get_cat_list() {
    //     let ncon = establish_connection();
    //     let cats = get_categories(&ncon);
    //     assert!(cats.len() >= 0);
    // }

    // #[test]
    // fn new_user() {
    //     let ncon = establish_connection();
    //     let user = create_user(&ncon, "tester@test.com", "12345");
    //     assert!(user.email == "tester@test.com");
    // }
}
