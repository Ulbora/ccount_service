use ccount_service::database::category_db::create_category;
use ccount_service::database::category_db::delete_category;
use ccount_service::database::food_db::create_food;
use ccount_service::database::food_db::delete_food;
use ccount_service::database::food_db::get_food_by_category;
use ccount_service::database::food_db::update_food;
use ccount_service::establish_connection;

// #[cfg(test)]
mod tests {
    use super::*;
    use ccount_service::database::user_db::create_user;
    use ccount_service::database::user_db::delete_user;

    #[test]
    fn new_food() {
        let ncon = establish_connection();
        let ncat = create_category(&ncon, "test snacks");
        assert_ne!(ncat.id, 0);

        let user = create_user(&ncon, "newinttester2@test.com", "12345");
        assert!(user.email == "newinttester2@test.com");

        let user_slice: &str = &*user.email;
        let nfd = create_food(&ncon, "chips", ncat.id, 254, user_slice);
        assert_eq!(nfd.name, "chips");

        let user_slice: &str = &*user.email;
        let ufd = update_food(&ncon, nfd.id, "corn chips", ncat.id, 234, user_slice);
        assert_eq!(ufd.name, "corn chips");
        assert_eq!(ufd.calories, 234);

        let user_slice: &str = &*user.email;
        let nfd2 = create_food(&ncon, "crackers", ncat.id, 354, user_slice);
        assert_eq!(nfd2.name, "crackers");

        let user_slice: &str = &*user.email;
        let fds = get_food_by_category(&ncon, ncat.id, user_slice);
        assert!(fds.len() == 2);

        let cnt = delete_food(&ncon, nfd.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_category(&ncon, ncat.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_user(&ncon, "newinttester2@test.com");
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
