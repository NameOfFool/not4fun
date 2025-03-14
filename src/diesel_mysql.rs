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
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};
use rocket_db_pools::{Connection, Database};

type Result<T, E = Debug<rocket_db_pools::diesel::result::Error>> = std::result::Result<T, E>;
#[derive(Database)]
#[database("not4fun")]
struct Db(MysqlPool);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
struct User {
    id: i64,
    username: String,
    password: String,
}

table! {
    users(id){
        id ->BigInt,
        username -> Text,
        password -> Text
    }
}
#[get("/")]
async fn get_users(mut db: Connection<Db>) -> Result<Json<Vec<String>>> {
    let urs = users::table.select(users::username).load(&mut *db).await?;
    Ok(Json(urs))
}
#[post("/", data = "<user>")]
async fn create_user(mut db: Connection<Db>, mut user: Json<User>) -> Result<Created<Json<User>>> {
    diesel::sql_function!(fn last_inserted_id() -> BigInt);
    let user = db.transaction(|mut conn| Box::pin(async move {
        diesel::insert_into(users::table)
            .values(&*user)
            .execute(&mut conn)
            .await?;
        /*user.id = users::table
            .select(last_inserted_id())
            .first(&mut conn)
            .await?;*/

        Ok::<_, diesel::result::Error>(user)
    })).await?;

    Ok(Created::new("/").body(user))
}
fn get_salt<'a>(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel MySQL Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .mount("/users", routes![get_users, create_user])
    })
}
