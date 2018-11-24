#![feature(proc_macro_hygiene, decl_macro, try_from, transpose_result)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate postgres;
extern crate postgres_shared;
extern crate r2d2;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate r2d2_postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate argon2;
extern crate rand;
extern crate serde_json;
extern crate uuid;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod auth;
mod store;

fn main() -> Result<(), Box<::std::error::Error + Send + Sync>> {
    env_logger::init();

    info!("building rocket...");

    let rocket = ::rocket::ignite();
    let store = store::Store::from_config(rocket.config())?;
    rocket.manage(store).mount("/auth", auth::routes()).launch();

    Ok(())
}
