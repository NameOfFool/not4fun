mod diesel_mysql;

#[macro_use] extern crate rocket;

use rocket::response::Debug;
use rocket::serde::{Deserialize, Serialize, json::Json};
#[get("/")]
fn index() -> &'static str {
    "ABOBa"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(diesel_mysql::stage())
}
