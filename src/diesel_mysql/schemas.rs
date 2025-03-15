use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    users(id){
        id ->BigInt,
        username -> Text,
        email -> Text,
        password -> Text
    }
}

table!{
    posts(id){
        id -> BigInt,
        author_id->BigInt,
        content ->Text
    }
}
joinable!(posts -> users (author_id));
allow_tables_to_appear_in_same_query!(users, posts);
