use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub fingerprint: String,
    pub theme_id: String,
    pub timezone: String,
    pub bio: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            fingerprint: row.get("fingerprint"),
            theme_id: row.get("theme_id"),
            timezone: row.get("timezone"),
            bio: row.get("bio"),
            is_admin: row.get("is_admin"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
