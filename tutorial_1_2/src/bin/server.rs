use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageData {
    from: String,
    message: String,
}

type Users = Arc<Mutex<HashMap<SocketAddr, String>>>;

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Users,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            let parsed: WebSocketMessage = match serde_json::from_str(text) {
                                Ok(m) => m,
                                Err(_) => continue,
                            };

                            match parsed.message_type {
                                MsgTypes::Register => {
                                    let username = parsed.data.unwrap_or(addr.to_string());
                                    println!("{addr} registered as {username}");
                                    users.lock().unwrap().insert(addr, username);

                                    // Broadcast updated user list ke semua
                                    let user_list: Vec<String> = users.lock().unwrap().values().cloned().collect();
                                    let response = WebSocketMessage {
                                        message_type: MsgTypes::Users,
                                        data_array: Some(user_list),
                                        data: None,
                                    };
                                    bcast_tx.send(serde_json::to_string(&response).unwrap())?;
                                }
                                MsgTypes::Message => {
                                    let from = users.lock().unwrap()
                                        .get(&addr).cloned()
                                        .unwrap_or(addr.to_string());
                                    let message_data = MessageData {
                                        from,
                                        message: parsed.data.unwrap_or_default(),
                                    };
                                    let response = WebSocketMessage {
                                        message_type: MsgTypes::Message,
                                        data: Some(serde_json::to_string(&message_data).unwrap()),
                                        data_array: None,
                                    };
                                    bcast_tx.send(serde_json::to_string(&response).unwrap())?;
                                }
                                _ => {}
                            }
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                        // Client disconnect, hapus dari users list
                        users.lock().unwrap().remove(&addr);
                        let user_list: Vec<String> = users.lock().unwrap().values().cloned().collect();
                        let response = WebSocketMessage {
                            message_type: MsgTypes::Users,
                            data_array: Some(user_list),
                            data: None,
                        };
                        let _ = bcast_tx.send(serde_json::to_string(&response).unwrap());
                        return Ok(());
                    }
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();

        tokio::spawn(async move {
            let ws_stream = ServerBuilder::new().accept(socket).await.unwrap();
            if let Err(e) = handle_connection(addr, ws_stream, bcast_tx, users).await {
                println!("Error: {e}");
            }
        });
    }
}