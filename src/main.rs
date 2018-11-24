#![feature(proc_macro_hygiene, decl_macro, try_from, transpose_result)]

#[macro_use]
extern crate log;
extern crate postgres;
extern crate postgres_shared;
extern crate pretty_env_logger;
extern crate r2d2;
#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_derive;
extern crate uuid;

mod auth;
mod store;

fn main() -> Result<(), Box<::std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    info!("building rocket...");

    let rocket = ::rocket::ignite();
    let store = store::Store::from_config(rocket.config())?;
    rocket.manage(store);

    Ok(())
}
