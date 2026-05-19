use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CatMood {
    Happy,
    Sleeping,
    Hungry,
    Thirsty,
    Bored,
    Sad,
}

impl CatMood {
    pub fn as_str(&self) -> &'static str {
        match self {
            CatMood::Happy => "happy",
            CatMood::Sleeping => "sleeping",
            CatMood::Hungry => "hungry",
            CatMood::Thirsty => "thirsty",
            CatMood::Bored => "bored",
            CatMood::Sad => "sad",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cat {
    pub id: Uuid,
    pub user_id: Uuid,
    pub growth_points: i32,
    pub last_cared_at: Option<DateTime<Utc>>,
    pub seed: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Cat {
    pub fn from_row(row: &tokio_postgres::Row) -> Self {
        Cat {
            id: row.get("id"),
            user_id: row.get("user_id"),
            growth_points: row.get("growth_points"),
            last_cared_at: row.get("last_cared_at"),
            seed: row.get("seed"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn stage(&self) -> u8 {
        match self.growth_points {
            0..=99 => 0,
            100..=199 => 1,
            200..=299 => 2,
            300..=399 => 3,
            400..=499 => 4,
            500..=599 => 5,
            _ => 6,
        }
    }

    pub fn mood(&self) -> CatMood {
        let now = Utc::now();
        let hours_since_care = self.last_cared_at.map(|t| {
            (now - t).num_hours()
        }).unwrap_or(999);

        match hours_since_care {
            0..=6 => CatMood::Happy,
            7..=12 => CatMood::Bored,
            13..=24 => CatMood::Hungry,
            25..=48 => CatMood::Thirsty,
            _ => CatMood::Sad,
        }
    }

    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }
}
