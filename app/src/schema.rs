// @generated automatically by Diesel CLI.

diesel::table! {
    to_do (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        date -> Timestamp,
        user_id -> Int4,
    }
}
diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id -> Varchar,
    }
}

joinable!(to_do -> users (user_id));
allow_tables_to_appear_in_same_query!(
    to_do,
    users,
);
