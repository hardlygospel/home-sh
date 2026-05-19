use anyhow::{Context, Result};
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
                   WHERE user_id = $1
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
                   WHERE user_id = $1
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
                   WHERE user_id = $1
                   RETURNING *"#,
                &[&user_id],
            )
            .await
            .context("Failed to groom cat")?;
        Ok(Cat::from_row(&row))
    }

    pub async fn tick_passive_growth(&self) -> Result<()> {
        let client = self.pool.get().await.context("Failed to get DB client")?;
        client
            .execute(
                r#"UPDATE home_cat_trees
                   SET growth_points = LEAST(growth_points + 1, 700),
                       updated_at = NOW()
                   WHERE last_cared_at > NOW() - INTERVAL '24 hours'"#,
                &[],
            )
            .await
            .context("Failed to tick passive growth")?;
        Ok(())
    }
}
