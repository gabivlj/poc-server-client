use futures_util::{SinkExt, StreamExt};
use log::*;
use rawmad_common::{HelloMessage, Message, RawmadError};
use tokio_tungstenite::connect_async;
use url::Url;

async fn run() -> Result<(), RawmadError> {
    let (mut ws_stream, _) = loop {
        let case_url =
            Url::parse(&format!("ws://localhost:9002"))
             .expect("bad URL");
        let res = connect_async(case_url).await;
        match res {
            Ok(stream) => break stream,
            Err(err) => {
                if let tungstenite::Error::Io(io_err) = &err {
                    if io_err.kind() == std::io::ErrorKind::ConnectionRefused {
                        warn!("server not yet up, retrying again in 1s...");
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        continue;
                    }
                }

                return Err(RawmadError::Ws(err));
            }
        }
    };

    info!("sending hello to the server");   
    ws_stream.send(Message::Hello(HelloMessage { version:1, raw: String::from("hello!") }).try_into()?).await.map_err(|err| RawmadError::SendingMessage(err))?;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.map_err(|err| RawmadError::Ws(err))?;
        info!("receiving some message from server: {:?}", Message::try_from(msg)?);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    run().await.expect("program finish");
}