#![allow(clippy::option_option)]

use crate::store::Uuid;
use crate::Result;
use chrono::{DateTime, Utc};
use postgres::rows::Row;
use postgres::GenericConnection;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invite {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub granter_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Invite {
    pub fn create_table<C: GenericConnection>(conn: &C) -> Result<()> {
        conn.batch_execute("
            CREATE TABLE IF NOT EXISTS invite (
                id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
                creator_id UUID NOT NULL REFERENCES users ON DELETE RESTRICT ON UPDATE RESTRICT,
                granter_id UUID REFERENCES users ON DELETE RESTRICT ON UPDATE RESTRICT,
                user_id UUID REFERENCES users ON DELETE CASCADE ON UPDATE CASCADE,
                active BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE SET DEFAULT
            )
        ")?;

        Ok(())
    }

    pub fn create<C: GenericConnection>(
        conn: &C,
        creator_id: Uuid,
        granter_id: Option<Uuid>,
        user_id: Option<Uuid>,
        active: bool,
    ) -> Result<Option<Invite>> {
        Ok(conn.query("INSERT INTO invites (creator_id, granter_id, user_id, active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $5) RETURNING *", 
            &[&creator_id, &granter_id, &user_id, &active, &Utc::now()])?
            .iter().next().map(Invite::from))
    }

    pub fn find<C: GenericConnection, U: Into<Uuid>>(conn: &C, id: U) -> Result<Option<Invite>> {
        conn.query("SELECT * FROM invites WHERE id = $1 LIMIT 1", &[&id.into()])
            .map(|r| r.iter().next().map(Invite::from))
            .map_err(|e| e.into())
    }

    pub fn find_update<C: GenericConnection, U: Into<Uuid>>(
        conn: &C,
        id: U,
    ) -> Result<Option<Invite>> {
        conn.query(
            "SELECT * FROM invites WHERE id = $1 LIMIT 1 FOR UPDATE",
            &[&id.into()],
        )
        .map(|r| r.iter().next().map(Invite::from))
        .map_err(|e| e.into())
    }

    pub fn usable(&self) -> bool {
        self.active && self.granter_id.is_some() && self.user_id.is_none()
    }

    pub fn update<C: GenericConnection, U: Into<Uuid>>(
        self,
        conn: &C,
        creator_id: Option<U>,
        granter_id: Option<Option<U>>,
        user_id: Option<Option<U>>,
        active: Option<bool>,
    ) -> Result<Invite> {
        let creator_id = creator_id.map(|u| u.into()).unwrap_or(self.creator_id);
        let granter_id = granter_id
            .map(|v| v.map(|u| u.into()))
            .unwrap_or(self.granter_id);
        let user_id = user_id.map(|v| v.map(|u| u.into())).unwrap_or(self.user_id);
        let active = active.unwrap_or(self.active);
        conn.query("UPDATE invites SET creator_id = $2, granter_id = $3, user_id = $4, active = $5, updated_at = CURRENT_TIMESTAMP WHERE id = $1", &[&self.id, &creator_id, &granter_id, &user_id, &active])
            .map(|r| r.iter().next().map(Invite::from).unwrap())
            .map_err(|e| e.into())
    }
}

impl<'a> From<Row<'a>> for Invite {
    fn from(row: Row<'a>) -> Invite {
        Invite {
            id: row.get("id"),
            creator_id: row.get("creator_id"),
            granter_id: row.get("granter_id"),
            user_id: row.get("user_id"),
            active: row.get("active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
