use actix_web::{web, App, HttpServer, HttpResponse, Error, post};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod websocket;
mod jupiter_api;

// Shared state for our application
struct AppState {
    connected_clients: HashMap<String, actix::Addr<websocket::PriceSocket>>,
    last_fetch: HashMap<String, DateTime<Utc>>,
    cached_prices: HashMap<String, f64>,
}

#[derive(Deserialize)]
struct TokenRequest {
    addresses: Vec<String>,
}

#[derive(Serialize)]
struct TokenResponse {
    prices: HashMap<String, f64>,
}

// WebSocket route handler
async fn ws_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    app_state: web::Data<Arc<Mutex<AppState>>>,
) -> Result<HttpResponse, Error> {
    let client_id = uuid::Uuid::new_v4().to_string();
    ws::start(
        websocket::PriceSocket::new(app_state.get_ref().clone(), client_id),
        &req,
        stream,
    )
}

// Background task for fetching and broadcasting token prices
async fn price_update_task(app_state: Arc<Mutex<AppState>>) {
    let mut interval = interval(Duration::from_secs(10));
    let token_addresses = vec![
        "So11111111111111111111111111111111111111112".to_string(), // SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(), // USDC
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(), // USDT
    ];

    loop {
        interval.tick().await;
        if let Ok(prices) = jupiter_api::fetch_token_prices(&token_addresses).await {
            let mut state = app_state.lock().await;
            for (address, price) in prices {
                state.last_fetch.insert(address.clone(), Utc::now());
                state.cached_prices.insert(address.clone(), price);
                
                for (_, client) in &state.connected_clients {
                    client.do_send(websocket::PriceUpdate(jupiter_api::TokenPrice { address: address.clone(), price }));
                }
            }
        }
    }
}

// POST endpoint to get prices for specified token addresses
#[post("/prices")]
async fn get_prices(
    app_state: web::Data<Arc<Mutex<AppState>>>,
    token_request: web::Json<TokenRequest>,
) -> HttpResponse {
    let addresses = &token_request.addresses;
    
    // First, check the cache for existing prices
    let mut response_prices = HashMap::new();
    {
        let state = app_state.lock().await;
        for address in addresses {
            if let Some(price) = state.cached_prices.get(address) {
                response_prices.insert(address.clone(), *price);
            }
        }
    }
    
    // For any missing prices, fetch from the API
    let missing_addresses: Vec<String> = addresses
        .iter()
        .filter(|addr| !response_prices.contains_key(*addr))
        .cloned()
        .collect();
    
    if !missing_addresses.is_empty() {
        if let Ok(new_prices) = jupiter_api::fetch_token_prices(&missing_addresses).await {
            let mut state = app_state.lock().await;
            for (address, price) in new_prices {
                state.last_fetch.insert(address.clone(), Utc::now());
                state.cached_prices.insert(address.clone(), price);
                response_prices.insert(address, price);
            }
        }
    }
    
    HttpResponse::Ok().json(TokenResponse { prices: response_prices })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize shared state
    let app_state = web::Data::new(Arc::new(Mutex::new(AppState {
        connected_clients: HashMap::new(),
        last_fetch: HashMap::new(),
        cached_prices: HashMap::new(),
    })));

    // Spawn the background task
    let state_clone = app_state.clone();
    tokio::spawn(async move {
        price_update_task(state_clone.get_ref().clone()).await;
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_prices)
            .route("/ws", web::get().to(ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}