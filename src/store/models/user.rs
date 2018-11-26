use crate::store::secret::Secret;
use crate::store::Connection;
use crate::store::Uuid;
use crate::Result;
use chrono::{DateTime, Utc};
use postgres::rows::Row;

pub struct User {
    pub id: Uuid,
    pub email: String,
    password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn create_table<C: Connection>(conn: &C) -> Result<()> {
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
    }

    pub fn create<C: Connection, E: AsRef<str>>(
        conn: &C,
        email: E,
        password: Secret,
    ) -> Result<Option<User>> {
        let result = conn
            .prepare("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *")?
            .query(&[&email.as_ref(), &password.as_ref()])?
            .iter()
            .next()
            .map(User::from);
        Ok(result)
    }

    pub fn find<C: Connection, U: Into<Uuid>>(conn: &C, id: U) -> Result<Option<User>> {
        let result = conn
            .prepare("SELECT * FROM users WHERE id = $1 AND valid = TRUE LIMIT 1")?
            .query(&[&id.into()])?
            .iter()
            .next()
            .map(User::from);
        Ok(result)
    }

    pub fn find_email<C: Connection, A: AsRef<str>>(conn: &C, email: A) -> Result<Option<User>> {
        let result = conn
            .prepare("SELECT * FROM users WHERE email = $1 LIMIT 1")?
            .query(&[&email.as_ref()])?
            .iter()
            .next()
            .map(User::from);
        Ok(result)
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
