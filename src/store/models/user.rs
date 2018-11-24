use crate::store::secret::Secret;
use crate::store::Store;
use chrono::{DateTime, Utc};
use postgres::rows::Row;
use std::error::Error;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: String,
    password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create(store: &Store) -> Result<(), Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            conn.batch_execute("
                CREATE TABLE IF NOT EXISTS users (
                    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
                    email TEXT UNIQUE NOT NULL,
                    password TEXT NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE SET DEFAULT
                )
            ")?;
            Ok(())
        })
    }

    pub fn find(id: Uuid, store: &Store) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            Ok(conn
                .prepare("SELECT * FROM users WHERE id = $1 AND valid = TRUE LIMIT 1")?
                .query(&[&id])?
                .iter()
                .next()
                .map(User::from))
        })
    }

    pub fn find_email<A: AsRef<str>>(
        email: A,
        store: &Store,
    ) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            Ok(conn
                .prepare("SELECT * FROM users WHERE email = $1 LIMIT 1")?
                .query(&[&email.as_ref()])?
                .iter()
                .next()
                .map(User::from))
        })
    }

    pub fn pass(&self) -> Secret<'_> {
        Secret::from(&self.password[..])
    }
}

impl From<Row<'_>> for User {
    fn from(row: Row<'_>) -> User {
        User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&'_ Row<'_>> for User {
    fn from(row: &'_ Row<'_>) -> User {
        User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
