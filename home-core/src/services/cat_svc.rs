use anyhow::{Context, Result};
use chrono::Utc;
use deadpool_postgres::Pool;
use rand::Rng;
use uuid::Uuid;

use crate::models::cat::Cat;

#[derive(Clone)]
pub struct CatService {
    pool: Pool,
}

impl CatService {
    pub fn new(pool: Pool) -> Self {
        CatService { pool }
    }

    pub async fn get_or_create_cat(&self, user_id: Uuid) -> Result<Cat> {
        let client = self.pool.get().await.context("Failed to get DB client")?;

        let row = client
            .query_opt(
                "SELECT * FROM home_cat_trees WHERE user_id = $1",
                &[&user_id],
            )
            .await
            .context("Failed to query cat")?;

        if let Some(row) = row {
            return Ok(Cat::from_row(&row));
        }

        let seed: i64 = rand::thread_rng().gen();
        let row = client
            .query_one(
                "INSERT INTO home_cat_trees (user_id, seed) VALUES ($1, $2) RETURNING *",
                &[&user_id, &seed],
            )
            .await
            .context("Failed to create cat")?;

        Ok(Cat::from_row(&row))
    }

    pub async fn feed(&self, user_id: Uuid) -> Result<Cat> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let row = client
            .query_one(
                r#"UPDATE home_cat_trees
                   SET last_cared_at = NOW(),
                       growth_points = LEAST(growth_points + 10, 700),
                       updated_at = NOW()
                   WHERE user_id = $1 AND is_alive = true
                   RETURNING *"#,
                &[&user_id],
            )
            .await
            .context("Failed to feed cat")?;
        Ok(Cat::from_row(&row))
    }

    pub async fn water(&self, user_id: Uuid) -> Result<Cat> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let row = client
            .query_one(
                r#"UPDATE home_cat_trees
                   SET last_cared_at = NOW(),
                       growth_points = LEAST(growth_points + 5, 700),
                       updated_at = NOW()
                   WHERE user_id = $1 AND is_alive = true
                   RETURNING *"#,
                &[&user_id],
            )
            .await
            .context("Failed to water cat")?;
        Ok(Cat::from_row(&row))
    }

    pub async fn groom(&self, user_id: Uuid) -> Result<Cat> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        let row = client
            .query_one(
                r#"UPDATE home_cat_trees
                   SET last_cared_at = NOW(),
                       growth_points = LEAST(growth_points + 7, 700),
                       updated_at = NOW()
                   WHERE user_id = $1 AND is_alive = true
                   RETURNING *"#,
                &[&user_id],
            )
            .await
            .context("Failed to groom cat")?;
        Ok(Cat::from_row(&row))
    }

    pub async fn kill_neglected_cats(&self) -> Result<u64> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        // Find cats that haven't been cared for in 7+ days
        let neglected = client
            .query(
                r#"SELECT id, user_id, created_at FROM home_cat_trees
                   WHERE is_alive = true
                   AND (
                       (last_cared_at IS NOT NULL AND last_cared_at < NOW() - INTERVAL '7 days')
                       OR (last_cared_at IS NULL AND created_at < NOW() - INTERVAL '7 days')
                   )"#,
                &[],
            )
            .await
            .context("Failed to query neglected cats")?;

        let count = neglected.len() as u64;
        for row in &neglected {
            let cat_id: Uuid = row.get("id");
            let user_id: Uuid = row.get("user_id");
            let created_at: chrono::DateTime<Utc> = row.get("created_at");
            let survived_days = (Utc::now() - created_at).num_days() as i32;

            client
                .execute(
                    "UPDATE home_cat_trees SET is_alive = false, updated_at = NOW() WHERE id = $1",
                    &[&cat_id],
                )
                .await
                .context("Failed to mark cat dead")?;

            client
                .execute(
                    "INSERT INTO home_cat_graveyard (user_id, survived_days) VALUES ($1, $2)",
                    &[&user_id, &survived_days],
                )
                .await
                .context("Failed to add to graveyard")?;
        }

        Ok(count)
    }

    pub async fn tick_passive_growth(&self) -> Result<()> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        // Passive growth: 1 point per hour for cared-for cats
        client
            .execute(
                r#"UPDATE home_cat_trees
                   SET growth_points = LEAST(growth_points + 1, 700),
                       updated_at = NOW()
                   WHERE is_alive = true
                   AND last_cared_at > NOW() - INTERVAL '24 hours'"#,
                &[],
            )
            .await
            .context("Failed to tick passive growth")?;
        Ok(())
    }
}
