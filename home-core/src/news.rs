use anyhow::{Context, Result};

use crate::models::article::Article;

const HN_TOP_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";
const HN_ITEM_URL: &str = "https://hacker-news.firebaseio.com/v0/item";

pub async fn fetch_top_stories(count: usize) -> Result<Vec<Article>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .context("Failed to build HTTP client")?;

    let ids: Vec<u64> = client
        .get(HN_TOP_URL)
        .send()
        .await
        .context("Failed to fetch HN top stories")?
        .json()
        .await
        .context("Failed to parse HN story IDs")?;

    let ids = &ids[..count.min(ids.len())];

    let mut articles = Vec::with_capacity(ids.len());
    for &id in ids {
        let url = format!("{}/{}.json", HN_ITEM_URL, id);
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(val) = resp.json::<serde_json::Value>().await {
                let title = val.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("(no title)")
                    .to_string();
                let story_url = val.get("url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let score = val.get("score")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0);
                articles.push(Article { id, title, url: story_url, score });
            }
        }
    }

    Ok(articles)
}
