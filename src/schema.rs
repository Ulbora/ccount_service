table! {
    category (id) {
        id -> Bigint,
        name -> Varchar,
    }
}

table! {
    daily_calories (id) {
        id -> Bigint,
        day -> Varchar,
        user_email -> Varchar,
        food_id -> Bigint,
    }
}

table! {
    food (id) {
        id -> Bigint,
        name -> Varchar,
        calories -> Integer,
        user_email -> Varchar,
        category_id -> Bigint,
    }
}

table! {
    user (email) {
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(daily_calories -> food (food_id));
joinable!(daily_calories -> user (user_email));
joinable!(food -> category (category_id));
joinable!(food -> user (user_email));

allow_tables_to_appear_in_same_query!(category, daily_calories, food, user,);
