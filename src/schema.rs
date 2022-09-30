// @generated automatically by Diesel CLI.

diesel::table! {
    booleans (id) {
        id -> Text,
        value -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
