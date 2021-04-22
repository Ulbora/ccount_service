use crate::database::daily_calory_db::calories_for_multi_days;
use crate::MysqlConnection;
use std::error::Error;

use crate::DailyCalories;

use crate::database::daily_calory_db::create_daily_calories;
use crate::database::daily_calory_db::delete_daily_calories;
use crate::database::daily_calory_db::get_daily_calories;

use crate::database::food_db::get_food_by_id;

pub struct CaloryCount {
    pub calories: i32,
    pub day: String,
}

pub fn create_new_daily_calories(
    conn: &MysqlConnection,
    day: &str,
    uemail: &str,
    fid: i64,
) -> DailyCalories {
    let rtn = create_daily_calories(conn, day, uemail, fid);
    rtn
}

pub fn get_calories_list_for_day(
    conn: &MysqlConnection,
    uemail: &str,
    dday: &str,
) -> Vec<DailyCalories> {
    let dcl = get_daily_calories(conn, uemail, dday);
    dcl
}

pub fn get_calories_for_day(conn: &MysqlConnection, uemail: &str, dday: &str) -> i32 {
    let mut rtn = 0;
    let dcl = get_daily_calories(conn, uemail, dday);
    for dc in dcl.iter() {
        let fd = get_food_by_id(conn, dc.food_id);
        rtn += fd.calories;
    }
    rtn
}

pub fn get_calories_for_multi_days(
    conn: &MysqlConnection,
    uemail: &str,
    days: i32,
) -> Vec<CaloryCount> {
    let mut rtn = Vec::<CaloryCount>::new();

    let cal_list = calories_for_multi_days(conn, uemail, days);
    for cc in cal_list {
        let ccc = CaloryCount {
            calories: cc.total.parse::<i32>().unwrap(),
            day: cc.day,
        };
        rtn.push(ccc);
    }
    rtn
}

pub fn delete_existing_daily_calory(
    conn: &MysqlConnection,
    dcid: i64,
    uemail: &str,
) -> Result<usize, Box<dyn Error>> {
    let rtn = delete_daily_calories(conn, dcid, uemail);
    rtn
}

#[cfg(test)]
mod tests {
    use crate::daily_calories::create_new_daily_calories;
    use crate::daily_calories::get_calories_for_multi_days;
    use crate::database::category_db::create_category;
    use crate::database::user_db::create_user;
    use crate::database::user_db::delete_user;
    use crate::establish_pooled_connection;
    use crate::food::create_new_food;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn get_calories_for_multi_days_test() {
        let ncon = establish_pooled_connection();
        let cat = create_category(&ncon.get().unwrap(), "snacks123test");

        let uemail = "newinttester6thread@test.com";

        let user_slice: &str = uemail;
        let suc = create_user(&ncon.get().unwrap(), user_slice, "12345");
        assert!(suc == true);

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
        let ncal = create_new_daily_calories(&ncon.get().unwrap(), "01-25-2021", user_slice, fd.id);
        assert_eq!(ncal.day, "01-25-2021");

        let user_slice: &str = uemail;
        let ncal2 =
            create_new_daily_calories(&ncon.get().unwrap(), "01-25-2021", user_slice, fd2.id);
        assert_eq!(ncal2.day, "01-25-2021");

        let user_slice: &str = uemail;
        let ncal = create_new_daily_calories(&ncon.get().unwrap(), "01-26-2021", user_slice, fd.id);
        assert_eq!(ncal.day, "01-26-2021");

        let user_slice: &str = uemail;
        let ncal2 =
            create_new_daily_calories(&ncon.get().unwrap(), "01-26-2021", user_slice, fd2.id);
        assert_eq!(ncal2.day, "01-26-2021");

        let user_slice: &str = uemail;
        let caltot = get_calories_for_multi_days(&ncon.get().unwrap(), user_slice, 10);
        assert_eq!(caltot.len(), 3);
        let mut tot = 0;
        for c in caltot {
            tot += c.calories
        }
        assert_eq!(tot, 2424);

        let cnt = delete_user(&ncon.get().unwrap(), user_slice);
        assert!(cnt.unwrap() == 1);
    }
}
