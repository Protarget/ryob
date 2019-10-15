table! {
    posts (id) {
        id -> Int8,
        posted_in -> Int8,
        created_by -> Int8,
        created_at -> Timestamptz,
        content -> Text,
    }
}

table! {
    topics (id) {
        id -> Int8,
        title -> Text,
        created_by -> Int8,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        user_name -> Text,
        password_hash -> Text,
    }
}

joinable!(posts -> topics (posted_in));
joinable!(posts -> users (created_by));
joinable!(topics -> users (created_by));

allow_tables_to_appear_in_same_query!(
    posts,
    topics,
    users,
);
