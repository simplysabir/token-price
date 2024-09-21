use actix_web::{web, App, HttpServer, HttpResponse, Error, get};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

mod websocket;
mod jupiter_api;

// Shared state for our application
struct AppState {
    connected_clients: HashMap<String, actix::Addr<websocket::PriceSocket>>,
    last_fetch: HashMap<String, DateTime<Utc>>,
    cached_prices: HashMap<String, f64>,
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
            for price in prices {
                state.last_fetch.insert(price.address.clone(), Utc::now());
                state.cached_prices.insert(price.address.clone(), price.price);
                
                for (_, client) in &state.connected_clients {
                    client.do_send(websocket::PriceUpdate(price.clone()));
                }
            }
        }
    }
}

// Simple HTTP endpoint to get current prices
#[get("/prices")]
async fn get_prices(app_state: web::Data<Arc<Mutex<AppState>>>) -> HttpResponse {
    let state = app_state.lock().await;
    let prices = state.cached_prices.clone();
    HttpResponse::Ok().json(prices)
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