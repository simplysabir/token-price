use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPrice {
    pub token: String,
    pub price: f64,
}

pub async fn fetch_token_price(token: &str) -> Result<TokenPrice, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://price.jup.ag/v4/price?ids={}", token);
    
    let response = client.get(&url).send().await?;
    let price_data: serde_json::Value = response.json().await?;
    
    // Extract the price from the JSON response
    let price = price_data["data"][token]["price"]
        .as_f64()
        .unwrap_or(0.0);
    
    Ok(TokenPrice {
        token: token.to_string(),
        price,
    })
}