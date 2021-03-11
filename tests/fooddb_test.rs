use ccount_service::categorydb::create_category;
use ccount_service::categorydb::delete_category;
use ccount_service::establish_connection;
use ccount_service::fooddb::create_food;
use ccount_service::fooddb::delete_food;
use ccount_service::fooddb::get_food_by_category;
use ccount_service::fooddb::update_food;

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_food() {
        let ncon = establish_connection();
        let ncat = create_category(&ncon, "test snacks");
        assert_ne!(ncat.id, 0);

        let nfd = create_food(&ncon, "chips", ncat.id, 254);
        assert_eq!(nfd.name, "chips");

        let ufd = update_food(&ncon, nfd.id, "corn chips", ncat.id, 234);
        assert_eq!(ufd.name, "corn chips");
        assert_eq!(ufd.calories, 234);

        let nfd2 = create_food(&ncon, "crackers", ncat.id, 354);
        assert_eq!(nfd2.name, "crackers");

        let fds = get_food_by_category(&ncon, ncat.id);
        assert!(fds.len() == 2);

        let cnt = delete_food(&ncon, nfd.id);
        assert!(cnt.unwrap() == 1);

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
