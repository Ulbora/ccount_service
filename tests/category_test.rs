use ccount_service::establish_pooled_connection;

use ccount_service::category::add_new_category;
use ccount_service::category::delete_existing_category;
use ccount_service::category::get_category_list;
use ccount_service::category::update_existing_category;

mod tests {
    use super::*;

    #[test]
    fn new_category() {
        let ncon = establish_pooled_connection();
        let cat = add_new_category(&ncon.get().unwrap(), "beans");
        assert!(cat.name == "beans");

        let cat = update_existing_category(&ncon.get().unwrap(), cat.id, "black beans");
        assert!(cat.name == "black beans");

        let cats = get_category_list(&ncon.get().unwrap());
        assert!(cats.len() >= 1);

        let res = delete_existing_category(&ncon.get().unwrap(), cat.id);
        assert!(res.unwrap() == 1);
    }
}
