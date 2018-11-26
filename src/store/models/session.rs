use crate::store::models::User;
use crate::store::Uuid;
use crate::Result;
use chrono::{DateTime, Utc};
use postgres::rows::Row;
use postgres::GenericConnection;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub kind: u32,
    pub user_id: Uuid,
    pub valid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SessionKind {
    Invalid = 0,
    Login = 1,
    Grant = 2,
}

impl Session {
    pub fn create_table<C: GenericConnection>(conn: &C) -> Result<()> {
        conn.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
                kind INTEGER NOT NULL DEFAULT 1,
                user_id UUID NOT NULL REFERENCES users ON DELETE CASCADE ON UPDATE RESTRICT,
                valid BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        ",
        )?;

        Ok(())
    }

    pub fn create<C: GenericConnection, U: Into<Uuid>>(
        conn: &C,
        kind: SessionKind,
        user_id: U,
        valid: bool,
    ) -> Result<Option<Session>> {
        Ok(conn.prepare("INSERT INTO sessions (kind, user_id, valid, created_at, updated_at) VALUES ($1, $2, $3) RETURNING *")?
            .query(&[&(kind as u32), &user_id.into(), &valid])?
            .iter().next().map(Session::from))
    }

    pub fn find<C: GenericConnection, U: Into<Uuid>>(conn: &C, id: U) -> Result<Option<Session>> {
        Ok(conn
            .prepare("SELECT * FROM sessions WHERE id = $1 LIMIT 1")?
            .query(&[&id.into()])?
            .iter()
            .next()
            .map(Session::from))
    }

    pub fn find_with_user<C: GenericConnection, U: Into<Uuid>>(
        conn: &C,
        id: U,
    ) -> Result<Option<(Session, User)>> {
        match Session::find(conn, id) {
            Ok(Some(session)) => {
                User::find(conn, session.user_id).map(|opt| opt.map(|user| (session, user)))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn kind(&self) -> SessionKind {
        match self.kind {
            1 => SessionKind::Login,
            2 => SessionKind::Grant,
            _ => SessionKind::Invalid,
        }
    }

    pub fn authenticable(&self) -> bool {
        self.valid
            && match self.kind() {
                SessionKind::Login | SessionKind::Grant => true,
                _ => false,
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
