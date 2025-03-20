mod diesel_mysql;

#[macro_use] extern crate rocket;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .attach(diesel_mysql::stage())
        .attach(cors.to_cors().unwrap())
}
