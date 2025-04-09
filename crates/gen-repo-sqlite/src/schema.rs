// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Text,
        body -> Text,
        updated -> Timestamp,
        updated_by -> Text,
    }
}

diesel::table! {
    persons (id) {
        id -> Text,
        extracted -> Bool,
        body -> Text,
        updated -> Timestamp,
        updated_by -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    persons,
);
