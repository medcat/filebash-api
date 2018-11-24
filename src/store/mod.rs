use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::error::Error;

pub(crate) mod models;
pub(crate) mod secret;

#[derive(Clone)]
pub struct Store(r2d2::Pool<PostgresConnectionManager>);

impl Store {
    pub fn from_config(config: &rocket::Config) -> Result<Store, Box<dyn Error + Send + Sync>> {
        let conn = PostgresConnectionManager::new(config.get_str("database")?, TlsMode::None)?;
        Ok(Store(r2d2::Pool::new(conn)?))
    }

    pub fn with_connection<A, R>(&self, action: A) -> Result<R, Box<dyn Error + Send + Sync>>
    where
        A: FnOnce(&Connection) -> Result<R, Box<dyn Error + Send + Sync>>,
    {
        let conn = self.0.get()?;
        action(&*conn)
    }
}
