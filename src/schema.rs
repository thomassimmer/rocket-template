diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Nullable<Integer>,
        content -> Text,
        author_id -> Integer,
        creation_date -> Timestamp,
    }
}

diesel::joinable!(messages -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(users, messages,);
