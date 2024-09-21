use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenPrice {
    pub address: String,
    pub price: f64,
}

pub async fn fetch_token_prices(addresses: &[String]) -> Result<Vec<TokenPrice>, reqwest::Error> {
    let client = Client::new();
    let url = "https://price.jup.ag/v4/price";
    
    let query: HashMap<_, _> = [("ids", addresses.join(","))].into_iter().collect();
    
    let response = client.get(url).query(&query).send().await?;
    let price_data: serde_json::Value = response.json().await?;
    
    let mut prices = Vec::new();
    if let Some(data) = price_data["data"].as_object() {
        for (address, price_info) in data {
            if let Some(price) = price_info["price"].as_f64() {
                prices.push(TokenPrice {
                    address: address.to_string(),
                    price,
                });
            }
        }
    }
    
    Ok(prices)
}