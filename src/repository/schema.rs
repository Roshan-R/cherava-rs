// @generated automatically by Diesel CLI.

diesel::table! {
    workflows (id) {
        id -> Text,
        user -> Nullable<Text>,
        data -> Nullable<Text>,
        selector -> Nullable<Text>,
        cron -> Nullable<Text>,
        lastupdated -> Nullable<Int8>,
        url -> Nullable<Text>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
