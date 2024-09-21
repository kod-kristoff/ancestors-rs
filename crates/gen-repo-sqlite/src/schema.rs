// @generated automatically by Diesel CLI.

diesel::table! {
    persons (id) {
        id -> Text,
        extracted -> Bool,
        body -> Text,
        updated -> TimestamptzSqlite,
        updated_by -> Text,
    }
}
