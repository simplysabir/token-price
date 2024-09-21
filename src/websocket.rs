use actix::{Actor, StreamHandler, ActorContext, AsyncContext};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::AppState;

pub struct PriceSocket {
    app_state: Arc<Mutex<AppState>>,
}

impl PriceSocket {
    pub fn new(app_state: Arc<Mutex<AppState>>) -> Self {
        Self { app_state }
    }
}

impl Actor for PriceSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established");
        // Initialize the connection, e.g., start sending price updates
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PriceSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                // Handle incoming messages, e.g., subscribe to specific tokens
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}