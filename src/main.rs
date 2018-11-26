#![feature(proc_macro_hygiene, decl_macro, try_from, transpose_result)]
#![warn(clippy::all)]
#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate argon2;
extern crate chrono;
extern crate env_logger;
extern crate postgres;
extern crate postgres_shared;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate rand;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate zxcvbn;

#[macro_use]
mod store;
mod api;
mod auth;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    env_logger::init();

    info!("building rocket...");

    let rocket = ::rocket::ignite();
    let store = store::Store::from_config(rocket.config())?;
    store.create_tables()?;
    rocket
        .manage(store)
        .mount("/auth", auth::routes())
        .mount("/api", api::routes())
        .launch();

    Ok(())
}
