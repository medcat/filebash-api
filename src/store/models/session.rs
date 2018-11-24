use crate::store::models::User;
use crate::store::Store;
use chrono::{DateTime, Utc};
use postgres::rows::Row;
use std::error::Error;
use uuid::Uuid;

pub struct Session {
    pub id: Uuid,
    pub kind: u32,
    pub user_id: Uuid,
    pub valid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn create(store: &Store) -> Result<(), Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            conn.batch_execute("
                CREATE TABLE IF NOT EXISTS sessions (
                    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
                    kind INTEGER NOT NULL DEFAULT 1,
                    user_id UUID NOT NULL REFERENCES users ON DELETE CASCADE ON UPDATE RESTRICT,
                    valid BOOLEAN NOT NULL DEFAULT FALSE,
                    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE SET DEFAULT
                )
            ")?;

            Ok(())
        })
    }

    pub fn find(id: Uuid, store: &Store) -> Result<Option<Session>, Box<dyn Error + Send + Sync>> {
        store.with_connection(|conn| {
            Ok(conn
                .prepare("SELECT * FROM sessions WHERE id = $1 AND valid = TRUE LIMIT 1")?
                .query(&[&id])?
                .iter()
                .next()
                .map(Session::from))
        })
    }

    pub fn find_with_user(
        id: Uuid,
        store: &Store,
    ) -> Result<Option<(Session, User)>, Box<dyn Error + Send + Sync>> {
        match Session::find(id, store) {
            Ok(Some(session)) => {
                User::find(session.user_id, store).map(|opt| opt.map(|user| (session, user)))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl From<Row<'_>> for Session {
    fn from(row: Row<'_>) -> Session {
        Session {
            id: row.get("id"),
            kind: row.get("kind"),
            user_id: row.get("user_id"),
            valid: row.get("valid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&'_ Row<'_>> for Session {
    fn from(row: &Row<'_>) -> Session {
        Session {
            id: row.get("id"),
            kind: row.get("kind"),
            user_id: row.get("user_id"),
            valid: row.get("valid"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
