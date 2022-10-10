// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        time_deleted -> Nullable<Timestamptz>,
    }
}
