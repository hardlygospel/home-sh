use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

impl ChatRoom {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        ChatRoom {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            is_public: row.get("is_public"),
            created_at: row.get("created_at"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub body: String,
    pub reply_to_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        ChatMessage {
            id: row.get("id"),
            room_id: row.get("room_id"),
            user_id: row.get("user_id"),
            username: row.get("username"),
            body: row.get("body"),
            reply_to_id: row.get("reply_to_id"),
            created_at: row.get("created_at"),
        }
    }
}
