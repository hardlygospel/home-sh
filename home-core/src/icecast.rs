use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NowPlaying {
    pub title: String,
    pub artist: String,
    pub duration_secs: u64,
    pub listeners: u32,
    pub elapsed_secs: u64,
}

#[derive(Debug, Deserialize)]
struct IcecastResponse {
    icestats: IcecastStats,
}

#[derive(Debug, Deserialize)]
struct IcecastStats {
    source: Option<serde_json::Value>,
}

pub async fn fetch_now_playing(icecast_url: &str) -> Result<Option<NowPlaying>> {
    let url = format!("{}/status-json.xsl", icecast_url);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .context("Failed to build HTTP client")?;

    let resp = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch Icecast status")?;

    let data: IcecastResponse = resp
        .json()
        .await
        .context("Failed to parse Icecast response")?;

    let source = match data.icestats.source {
        Some(s) => s,
        None => return Ok(None),
    };

    // source can be an object or array
    let source_obj = if source.is_array() {
        source.as_array().and_then(|a| a.first()).cloned()
    } else {
        Some(source)
    };

    let source_obj = match source_obj {
        Some(s) => s,
        None => return Ok(None),
    };

    let title = source_obj
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Title often comes as "Artist - Track"
    let (artist, track_title) = if let Some(pos) = title.find(" - ") {
        let a = title[..pos].to_string();
        let t = title[pos + 3..].to_string();
        (a, t)
    } else {
        (String::new(), title.clone())
    };

    let listeners = source_obj
        .get("listeners")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    Ok(Some(NowPlaying {
        title: track_title,
        artist,
        duration_secs: 0,
        listeners,
        elapsed_secs: 0,
    }))
}
