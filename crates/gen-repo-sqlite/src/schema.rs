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
    documents (id) {
        id -> Text,
        body -> Text,
        updated -> Timestamp,
        updated_by -> Text,
    }
}

diesel::table! {
    household_members (household_id, person_id) {
        household_id -> Text,
        person_id -> Text,
        role -> Nullable<Text>,
        from_date -> Nullable<Text>,
        to_date -> Nullable<Text>,
    }
}

diesel::table! {
    households (id) {
        id -> Text,
        name -> Text,
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

diesel::joinable!(household_members -> households (household_id));
diesel::joinable!(household_members -> persons (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    documents,
    household_members,
    households,
    persons,
);
