use futures_util::{SinkExt, StreamExt};
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::Result;
use rawmad_common::{Message, RawmadError};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
           RawmadError::Ws(e) => {
                match e {
                    Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                    err => error!("error processing connection: {}", err),
               }
            },
            RawmadError::DecodingMessage(inner_err) => {
                error!("couldn't decode message because of error: {}", inner_err.as_ref());
            },
            RawmadError::SendingMessage(inner_err) => {
                error!("couldn't send message because of error: {}", inner_err);
            },
            RawmadError::SerializationError(err) => {
                error!("couldn't serialize message because of error: {}", err);
            }
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<(), RawmadError> {
    let mut ws_stream = accept_async(stream).await.map_err(|err| RawmadError::Ws(err))?;
    info!("new WebSocket connection: {}", peer);
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.map_err(|err| RawmadError::Ws(err))?;
        let message = Message::try_from(msg)?;
        match message {
            Message::Hello(msg) => {
                info!("received msg: {:?}", msg); 
                let msg_back = Message::Hello(rawmad_common::HelloMessage{
                    version: 1,
                    raw: String::from("hello back: ") + &msg.raw,
                }).try_into()?;
                ws_stream.send(msg_back).await.map_err(|err| RawmadError::SendingMessage(err))?;
            },
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("init server");
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}