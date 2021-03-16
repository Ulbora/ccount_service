use ccount_service::establish_pooled_connection;
use ccount_service::food::create_new_food;
use ccount_service::food::delete_existing_food;
use ccount_service::food::get_food_list_by_category;
use ccount_service::food::update_existing_food;

use ccount_service::database::category_db::create_category;
use ccount_service::database::category_db::delete_category;
use ccount_service::database::user_db::create_user;
use ccount_service::database::user_db::delete_user;

mod tests {
    use super::*;

    #[test]
    fn new_food() {
        let ncon = establish_pooled_connection();
        let cat = create_category(&ncon.get().unwrap(), "snacks");
        assert!(cat.name == "snacks");

        let suc = create_user(&ncon.get().unwrap(), "newinttester4@test.com", "12345");
        assert!(suc == true);

        let uemail = "newinttester4@test.com";
        let user_slice: &str = uemail;
        let fd = create_new_food(&ncon.get().unwrap(), "chips", cat.id, 254, user_slice);
        assert!(fd.name == "chips");

        let user_slice: &str = uemail;
        let fd2 = create_new_food(
            &ncon.get().unwrap(),
            "potato chips",
            cat.id,
            254,
            user_slice,
        );
        assert!(fd2.name == "potato chips");

        let user_slice: &str = uemail;
        let fd = update_existing_food(
            &ncon.get().unwrap(),
            fd.id,
            "corn chips",
            cat.id,
            244,
            user_slice,
        );
        assert!(fd.name == "corn chips");

        let user_slice: &str = uemail;
        let flst = get_food_list_by_category(&ncon.get().unwrap(), cat.id, user_slice);
        assert!(flst.len() == 2);

        let cnt = delete_existing_food(&ncon.get().unwrap(), fd.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_category(&ncon.get().unwrap(), cat.id);
        assert!(cnt.unwrap() == 1);

        let cnt = delete_user(&ncon.get().unwrap(), "newinttester4@test.com");
        assert!(cnt.unwrap() == 1);
    }
}
