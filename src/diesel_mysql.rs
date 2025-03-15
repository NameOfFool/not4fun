pub mod models;
pub mod schemas;
mod tests;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rocket::fairing::AdHoc;
use rocket::response::Debug;
use rocket::response::status::Created;
use rocket::serde::{json::Json};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};
use rocket_db_pools::{Connection, Database};
use crate::diesel_mysql::models::*;
use crate::diesel_mysql::schemas::*;

type Result<T, E = Debug<rocket_db_pools::diesel::result::Error>> = std::result::Result<T, E>;
#[derive(Database)]
#[database("not4fun")]
pub struct Db(MysqlPool);
//Users
#[get("/")]
async fn get_users(mut db: Connection<Db>) -> Result<Json<Vec<String>>> {
    let urs = users::table.select(users::username).load(&mut *db).await?;
    Ok(Json(urs))
}
#[post("/", data = "<user>")]
async fn create_user(mut db: Connection<Db>, mut user: Json<User>) -> Result<Created<Json<User>>> {
    let user = db.transaction(|mut conn| Box::pin(async move {
        user.password = get_salt(&user.password);
        diesel::insert_into(users::table)
            .values(&*user)
            .execute(&mut conn)
            .await?;

        Ok::<_, diesel::result::Error>(user)
    })).await?;

    Ok(Created::new("/").body(user))
}
#[delete("/<id>")]
async fn delete_user(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let affected = diesel::delete(users::table.find(id)).execute(&mut *db).await?;

    Ok((affected == 1).then(|| ()))
}

#[get("/<id>")]
async fn get_user(mut db: Connection<Db>, id: i64) -> Option<Json<User>> {
    users::table
        .filter(users::id.eq(id))
        .first(&mut db)
        .await
        .map(Json)
        .ok()
}
fn get_salt<'a>(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}
//EndUsers

//Posts

//EndPosts
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/users", routes![get_users, create_user, delete_user, get_user])
    })
}
