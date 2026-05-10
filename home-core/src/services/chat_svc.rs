use anyhow::{Context, Result};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::models::chat::{ChatMessage, ChatRoom};

#[derive(Clone)]
pub struct ChatService {
    pool: Pool,
}

impl ChatService {
    pub fn new(pool: Pool) -> Self {
        ChatService { pool }
    }

    pub async fn get_rooms(&self) -> Result<Vec<ChatRoom>> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let rows = client
            .query(
                "SELECT * FROM home_chat_rooms WHERE is_public = true ORDER BY name",
                &[],
            )
            .await
            .context("Failed to query rooms")?;
        Ok(rows.iter().map(ChatRoom::from_row).collect())
    }

    pub async fn get_messages(&self, room_id: Uuid, limit: i64) -> Result<Vec<ChatMessage>> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let rows = client
            .query(
                r#"SELECT m.id, m.room_id, m.user_id, u.username, m.body, m.reply_to_id, m.created_at
                   FROM home_chat_messages m
                   JOIN home_users u ON u.id = m.user_id
                   WHERE m.room_id = $1
                   ORDER BY m.created_at DESC
                   LIMIT $2"#,
                &[&room_id, &limit],
            )
            .await
            .context("Failed to query messages")?;

        let mut messages: Vec<ChatMessage> = rows.iter().map(ChatMessage::from_row).collect();
        messages.reverse();
        Ok(messages)
    }

    pub async fn send_message(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        body: &str,
        reply_to_id: Option<Uuid>,
    ) -> Result<ChatMessage> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let row = client
            .query_one(
                r#"WITH inserted AS (
                    INSERT INTO home_chat_messages (room_id, user_id, body, reply_to_id)
                    VALUES ($1, $2, $3, $4) RETURNING *
                )
                SELECT i.id, i.room_id, i.user_id, u.username, i.body, i.reply_to_id, i.created_at
                FROM inserted i
                JOIN home_users u ON u.id = i.user_id"#,
                &[&room_id, &user_id, &body, &reply_to_id],
            )
            .await
            .context("Failed to insert message")?;

        Ok(ChatMessage::from_row(&row))
    }
}
