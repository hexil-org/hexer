#![deny(clippy::all)]

use std::{env, io::Error};

use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message as WebSocketMessage;
use tokio_tungstenite::WebSocketStream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9450".to_string());

    let listener = TcpListener::bind(&addr).await?;

    eprintln!("Server started. Listening on {}.", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_tcp(stream));
    }
}

async fn handle_tcp(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();

    handle_websocket(ws_stream).await;
}

async fn handle_websocket(stream: WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>) {
    let (mut write, mut read) = stream.split();

    while let Some(Ok(msg)) = read.next().await {
        if let WebSocketMessage::Text(text) = msg {
            if text == "ping" {
                let pong = WebSocketMessage::Text("pong".to_owned());
                write.send(pong).await.unwrap();
            }
        }
    }
}
