// @generated automatically by Diesel CLI.

diesel::table! {
    group_assigned_users (id) {
        id -> Integer,
        group_id -> Integer,
        user_id -> Integer,
    }
}

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

diesel::joinable!(group_assigned_users -> grups (group_id));
diesel::joinable!(group_assigned_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_assigned_users,
    grups,
    users,
);
