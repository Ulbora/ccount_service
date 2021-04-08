use ccount_service::database::category_db::create_category;
use ccount_service::database::category_db::delete_category;
use ccount_service::database::daily_calory_db::create_daily_calories;
use ccount_service::database::daily_calory_db::delete_daily_calories;
use ccount_service::database::daily_calory_db::get_daily_calories;
use ccount_service::database::food_db::create_food;
use ccount_service::database::user_db::create_user;
use ccount_service::database::user_db::delete_user;
use ccount_service::establish_connection;

//use ccount_service::food_db::delete_food;
//use ccount_service::food_db::get_food_by_category;
//use ccount_service::food_db::update_food;

// #[cfg(test)]
mod tests {
    use super::*;
    use ccount_service::database::daily_calory_db::get_day_list;

    #[test]
    fn new_daily_cal() {
        let ncon = establish_connection();
        let ncat = create_category(&ncon, "test snacks");
        assert_ne!(ncat.id, 0);

        let suc = create_user(&ncon, "newinttester@test.com", "12345");
        assert!(suc == true);

        let uemail = "newinttester@test.com";
        let user_slice: &str = uemail;
        let nfd = create_food(&ncon, "chips", ncat.id, 254, user_slice);
        assert_eq!(nfd.name, "chips");

        let user_slice: &str = uemail;
        let nfd2 = create_food(&ncon, "crackers", ncat.id, 354, user_slice);
        assert_eq!(nfd2.name, "crackers");

        let user_slice: &str = uemail;
        let ncal = create_daily_calories(&ncon, "01-24-2021", user_slice, nfd.id);
        assert_eq!(ncal.day, "01-24-2021");

        let ncal2 = create_daily_calories(&ncon, "02-24-2021", user_slice, nfd2.id);
        assert_eq!(ncal2.day, "02-24-2021");

        let user_slice: &str = uemail;
        let ncal3 = create_daily_calories(&ncon, "03-24-2021", user_slice, nfd.id);
        assert_eq!(ncal3.day, "03-24-2021");

        let ncal4 = create_daily_calories(&ncon, "03-24-2021", user_slice, nfd2.id);
        assert_eq!(ncal4.day, "03-24-2021");

        // let ufd = update_food(&ncon, nfd.id, "corn chips", ncat.id, 234);
        // assert_eq!(ufd.name, "corn chips");
        // assert_eq!(ufd.calories, 234);

        // let nfd2 = create_food(&ncon, "crackers", ncat.id, 354);
        // assert_eq!(nfd2.name, "crackers");

        let dcs = get_daily_calories(&ncon, user_slice, "01-24-2021");
        assert!(dcs.len() == 1);

        let dcs = get_daily_calories(&ncon, user_slice, "03-24-2021");
        assert!(dcs.len() == 2);

        let dcs = get_day_list(&ncon, user_slice, 2);
        assert!(dcs.len() == 2);

        let dcs = get_day_list(&ncon, user_slice, 3);
        assert!(dcs.len() == 3);

        let dcs = get_day_list(&ncon, user_slice, 10);
        assert!(dcs.len() == 3);

        let cnt = delete_daily_calories(&ncon, ncal3.id, user_slice);
        assert!(cnt.unwrap() == 1);

        let cnt2 = delete_daily_calories(&ncon, ncal4.id, user_slice);
        assert!(cnt2.unwrap() == 1);

        let dcs2 = get_daily_calories(&ncon, user_slice, "03-24-2021");
        assert!(dcs2.len() == 0);

        let cnt = delete_category(&ncon, ncat.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_user(&ncon, "newinttester@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
