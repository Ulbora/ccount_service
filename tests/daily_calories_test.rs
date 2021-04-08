use ccount_service::establish_pooled_connection;
use ccount_service::food::create_new_food;

use ccount_service::database::category_db::create_category;
use ccount_service::database::category_db::delete_category;
use ccount_service::database::user_db::create_user;
use ccount_service::database::user_db::delete_user;

use ccount_service::daily_calories::create_new_daily_calories;
use ccount_service::daily_calories::delete_existing_daily_calory;
use ccount_service::daily_calories::get_calories_for_day;
use ccount_service::daily_calories::get_calories_for_multi_days;
mod tests {
    use super::*;
    use ccount_service::daily_calories::get_calories_list_for_day;

    #[test]
    fn new_calories() {
        let ncon = establish_pooled_connection();
        let cat = create_category(&ncon.get().unwrap(), "snacks");
        assert!(cat.name == "snacks");

        let uemail = "newinttester6@test.com";

        let suc = create_user(&ncon.get().unwrap(), "newinttester6@test.com", "12345");
        assert!(suc == true);

        let user_slice: &str = uemail;
        let fd = create_new_food(&ncon.get().unwrap(), "chips", cat.id, 254, user_slice);
        assert!(fd.name == "chips");

        let user_slice: &str = uemail;
        let fd2 = create_new_food(&ncon.get().unwrap(), "pizza", cat.id, 554, user_slice);
        assert!(fd2.name == "pizza");

        let user_slice: &str = uemail;
        let ncal = create_new_daily_calories(&ncon.get().unwrap(), "01-24-2021", user_slice, fd.id);
        assert_eq!(ncal.day, "01-24-2021");

        let user_slice: &str = uemail;
        let ncal2 =
            create_new_daily_calories(&ncon.get().unwrap(), "01-24-2021", user_slice, fd2.id);
        assert_eq!(ncal2.day, "01-24-2021");

        let user_slice: &str = uemail;
        let callst = get_calories_list_for_day(&ncon.get().unwrap(), user_slice, "01-24-2021");
        assert_eq!(callst.len(), 2);

        let user_slice: &str = uemail;
        let caltot = get_calories_for_day(&ncon.get().unwrap(), user_slice, "01-24-2021");
        assert_eq!(caltot, 808);

        let user_slice: &str = uemail;
        let caltot = get_calories_for_multi_days(&ncon.get().unwrap(), user_slice, 2);
        assert_eq!(caltot[0].calories, 808);

        let cnt2 = delete_existing_daily_calory(&ncon.get().unwrap(), ncal2.id, user_slice);
        assert!(cnt2.unwrap() == 1);

        let user_slice: &str = uemail;
        let caltot = get_calories_for_day(&ncon.get().unwrap(), user_slice, "01-24-2021");
        assert_eq!(caltot, 254);

        let cnt = delete_category(&ncon.get().unwrap(), cat.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_user(&ncon.get().unwrap(), "newinttester6@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
