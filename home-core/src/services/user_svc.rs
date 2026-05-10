use anyhow::{Context, Result};
use deadpool_postgres::Pool;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Clone)]
pub struct UserService {
    pool: Pool,
}

impl UserService {
    pub fn new(pool: Pool) -> Self {
        UserService { pool }
    }

    pub async fn get_or_create_user(&self, fingerprint: &str, username: &str) -> Result<User> {
        let client = self.pool.get().await.context("Failed to get DB client")?;

        // Try to find existing user
        let row = client
            .query_opt(
                "SELECT * FROM home_users WHERE fingerprint = $1",
                &[&fingerprint],
            )
            .await
            .context("Failed to query user")?;

        if let Some(row) = row {
            return Ok(User::from_row(&row));
        }

        // Create new user — ensure unique username
        let mut uname = username.to_string();
        let mut attempt = 0usize;
        loop {
            let candidate = if attempt == 0 {
                uname.clone()
            } else {
                format!("{}{}", username, attempt)
            };

            let existing = client
                .query_opt(
                    "SELECT id FROM home_users WHERE username = $1",
                    &[&candidate],
                )
                .await
                .context("Failed to check username")?;

            if existing.is_none() {
                uname = candidate;
                break;
            }
            attempt += 1;
        }

        let row = client
            .query_one(
                "INSERT INTO home_users (username, fingerprint) VALUES ($1, $2) RETURNING *",
                &[&uname, &fingerprint],
            )
            .await
            .context("Failed to create user")?;

        Ok(User::from_row(&row))
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let row = client
            .query_opt("SELECT * FROM home_users WHERE id = $1", &[&id])
            .await
            .context("Failed to query user")?;
        Ok(row.map(|r| User::from_row(&r)))
    }

    pub async fn update_theme(&self, user_id: Uuid, theme_id: &str) -> Result<()> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        client
            .execute(
                "UPDATE home_users SET theme_id = $1, updated_at = NOW() WHERE id = $2",
                &[&theme_id, &user_id],
            )
            .await
            .context("Failed to update theme")?;
        Ok(())
    }

    pub async fn update_bio(&self, user_id: Uuid, bio: &str) -> Result<()> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        client
            .execute(
                "UPDATE home_users SET bio = $1, updated_at = NOW() WHERE id = $2",
                &[&bio, &user_id],
            )
            .await
            .context("Failed to update bio")?;
        Ok(())
    }

    pub async fn update_timezone(&self, user_id: Uuid, timezone: &str) -> Result<()> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        client
            .execute(
                "UPDATE home_users SET timezone = $1, updated_at = NOW() WHERE id = $2",
                &[&timezone, &user_id],
            )
            .await
            .context("Failed to update timezone")?;
        Ok(())
    }

    pub async fn get_online_users(&self, user_ids: &[Uuid]) -> Result<Vec<User>> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let rows = client
            .query(
                "SELECT * FROM home_users WHERE id = ANY($1)",
                &[&user_ids],
            )
            .await
            .context("Failed to query online users")?;
        Ok(rows.iter().map(User::from_row).collect())
    }
}
