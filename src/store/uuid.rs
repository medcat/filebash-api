use postgres::types::{FromSql, IsNull, ToSql, Type};
use rocket_contrib::uuid as rocket_uuid;
use std::error::Error;
use uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Uuid(uuid::Uuid);

impl Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        self.0
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Uuid {
        Uuid(uuid)
    }
}

impl From<rocket_uuid::Uuid> for Uuid {
    fn from(uuid: rocket_uuid::Uuid) -> Uuid {
        Uuid(uuid.into_inner())
    }
}

use postgres::to_sql_checked;

impl FromSql for Uuid {
    fn from_sql(_: &Type, raw: &[u8]) -> Result<Uuid, Box<Error + Sync + Send>> {
        if raw.len() != 16 {
            return Err("invalid message length".into());
        }
        let mut bytes = [0; 16];
        bytes.copy_from_slice(raw);
        Ok(uuid::Uuid::from_bytes(bytes).into())
    }

    fn accepts(ty: &Type) -> bool {
        ty == &postgres::types::UUID
    }
}

impl ToSql for Uuid {
    fn to_sql(&self, _: &Type, w: &mut Vec<u8>) -> Result<IsNull, Box<Error + Sync + Send>> {
        w.extend_from_slice(self.0.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty == &postgres::types::UUID
    }

    to_sql_checked!();
}
