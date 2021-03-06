#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use routes::*;
use std::env;

mod db;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .mount("/api/v1", routes![index, new, show, delete, author, update])
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {
    rocket().launch();
}
