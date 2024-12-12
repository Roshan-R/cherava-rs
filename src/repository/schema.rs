// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        access_token -> Nullable<Varchar>,
    }
}

diesel::table! {
    workflows (id) {
        #[max_length = 255]
        id -> Varchar,
        user_id -> Int4,
        #[max_length = 255]
        data -> Varchar,
        #[max_length = 255]
        selector -> Varchar,
        #[max_length = 255]
        cron -> Varchar,
        lastupdated -> Nullable<Int8>,
        #[max_length = 255]
        url -> Varchar,
    }
}

diesel::joinable!(workflows -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    workflows,
);
