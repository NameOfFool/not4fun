use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use crate::diesel_mysql::schemas::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub username: String,
    pub email:String,
    pub password: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post{
    pub id: i64,
    pub author_id:i64,
    pub content:String
}