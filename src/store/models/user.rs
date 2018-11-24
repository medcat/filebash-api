use crate::store::Store;
use chrono::{DateTime, Utc};
use postgres::rows::Row;
use std::error::Error;
use uuid::Uuid;

pub struct User {
    id: Uuid,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn find_email<A: AsRef<str>>(
        email: A,
        store: Store,
    ) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            Ok(conn.prepare_cached("SELECT * FROM users WHERE email = $1")?
                .query(&[email.as_ref()])?)
        })
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
