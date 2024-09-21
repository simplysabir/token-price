use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;

mod jupiter_api;
mod websocket;

struct AppState {
    // counter: Arc<Mutex<usize>>,
}

async fn ws_route(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    app_state: web::Data<Arc<Mutex<AppState>>>,
) -> Result<HttpResponse, Error> {
    ws::start(websocket::PriceSocket::new(app_state.clone()), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize shared state
    let app_state = web::Data::new(Arc::new(Mutex::new(AppState {
            // Initialize state here
    })));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/ws", web::get().to(ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
