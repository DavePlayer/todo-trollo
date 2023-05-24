// @generated automatically by Diesel CLI.

diesel::table! {
    grups (id) {
        id -> Integer,
        name -> Text,
        creator -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        login -> Text,
        password -> Text,
        img_url -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(grups, users,);
