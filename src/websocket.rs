use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::AppState;
use crate::jupiter_api::TokenPrice;

pub struct PriceSocket {
    app_state: Arc<Mutex<AppState>>,
    client_id: String,
}

impl PriceSocket {
    pub fn new(app_state: Arc<Mutex<AppState>>, client_id: String) -> Self {
        Self { app_state, client_id }
    }
}

impl Actor for PriceSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established for client: {}", self.client_id);
        let addr = ctx.address();
        let app_state = self.app_state.clone();
        let client_id = self.client_id.clone();

        ctx.spawn(async move {
            let mut state = app_state.lock().await;
            state.connected_clients.insert(client_id, addr.clone());
            
            let cached_prices = state.cached_prices.clone();
            drop(state);

            for (address, price) in cached_prices {
                addr.do_send(PriceUpdate(TokenPrice { address, price }));
            }
        }.into_actor(self));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed for client: {}", self.client_id);
        let app_state = self.app_state.clone();
        let client_id = self.client_id.clone();

        ctx.spawn(async move {
            let mut state = app_state.lock().await;
            state.connected_clients.remove(&client_id);
        }.into_actor(self));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PriceSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message from client {}: {}", self.client_id, text);
                // Handle incoming messages if needed
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

#[derive(Message)]
#[rtype(result = "()")]
pub struct PriceUpdate(pub TokenPrice);

impl Handler<PriceUpdate> for PriceSocket {
    type Result = ();

    fn handle(&mut self, msg: PriceUpdate, ctx: &mut Self::Context) {
        let price_json = serde_json::to_string(&msg.0).unwrap();
        ctx.text(price_json);
    }
}