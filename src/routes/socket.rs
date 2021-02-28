use crate::routes::Responce;
use futures::{SinkExt, StreamExt};
use prometheus::IntGauge;
use serde::{Deserialize, Serialize};
use warp::{
    ws::{Message, WebSocket, Ws},
    Reply,
};

lazy_static! {
    static ref CONNECTED_CLIENTS: IntGauge = register_int_gauge!("connected_clients", "Connected Clients").expect("metric can be created");
}

pub async fn handler(ws: Ws) -> Responce<impl Reply> {
    Ok(ws.on_upgrade(move |socket| handle_connection(socket, )))
}

async fn handle_connection(ws: WebSocket) {
    let (mut sender, mut rcv) = ws.split();

    info!("Connection Opened;");
    CONNECTED_CLIENTS.inc();

    while let Some(event) = rcv.next().await {
        match event {
            Ok(msg) => {
                debug!(
                    "Message recieved from websocket; msg={:?}",
                    msg,                );
                if let Ok(txt) = msg.to_str() {
                    //handle message
                }
            }
            Err(err) => {
                error!(
                    "websocket error; error={}",
                    err
                );
                break;
            }
        }
    }
    info!("Connection Dropped;");
    CONNECTED_CLIENTS.dec();
}
