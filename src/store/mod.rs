use crate::Result;
use postgres::transaction::Transaction;
use postgres::Connection as PgConnection;
use postgres::GenericConnection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::error::Error;

pub(crate) mod models;
pub(crate) mod secret;
pub(crate) mod uuid;

#[macro_use]
pub(crate) mod outcome;

pub use self::outcome::{Outcome, OutcomeExt};
pub use self::secret::Secret;
pub use self::uuid::Uuid;

#[derive(Clone)]
pub struct Store(r2d2::Pool<PostgresConnectionManager>);

pub trait Connection: GenericConnection {}

impl<T: GenericConnection> Connection for T {}

impl Store {
    pub fn from_config(config: &rocket::Config) -> Result<Store> {
        let conn = PostgresConnectionManager::new(config.get_str("database")?, TlsMode::None)?;
        Ok(Store(r2d2::Pool::new(conn)?))
    }

    pub fn with_connection<A, R>(&self, action: A) -> Result<R>
    where
        A: FnOnce(&PgConnection) -> Result<R>,
    {
        let conn = self.0.get()?;
        action(&*conn)
    }

    pub fn with_transaction<C, R, O, A>(
        &self,
        action: A,
    ) -> Outcome<C, R, Box<dyn Error + Send + Sync>>
    where
        O: Into<Outcome<C, R, Box<dyn Error + Send + Sync>>>,
        A: FnOnce(&Transaction) -> O,
    {
        self.with_connection(|conn| {
            let trans = conn.transaction()?;
            match action(&trans).into() {
                Outcome::Commit(result) => {
                    trans.commit()?;
                    Ok(Outcome::Commit(result))
                }
                Outcome::Rollback(result) => {
                    trans.set_rollback();
                    trans.finish()?;
                    Ok(Outcome::Rollback(result))
                }
                Outcome::Err(error) => {
                    trans.set_rollback();
                    trans.finish()?;
                    Ok(Outcome::Err(error))
                }
            }
        })
        .unwrap_or_else(Outcome::Err)
    }

    pub fn create_tables(&self) -> Result<()> {
        self.with_connection(|conn| {
            models::User::create_table(conn)?;
            models::Session::create_table(conn)?;
            models::Invite::create_table(conn)?;
            Ok(())
        })
    }
}
