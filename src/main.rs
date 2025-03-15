mod diesel_mysql;

#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(diesel_mysql::stage())
}
