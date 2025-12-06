use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MarketMetadata {
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub image_url: Option<String>,
}

pub async fn fetch_market_metadata(metadata_url: &str) -> Result<MarketMetadata, reqwest::Error> {
    let http = Client::new();

    let response = http.get(metadata_url).send().await?;
    dbg!("Fetching market metadata from URL:", metadata_url);
    dbg!("response: {:?}", &response);
    let metadata = response.json::<MarketMetadata>().await?;
    dbg!("Fetched market metadata from URL:", metadata_url);
    dbg!("Market metadata:", &metadata);
    Ok(metadata)
}
