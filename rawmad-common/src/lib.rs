use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello(HelloMessage)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloMessage {
    pub version: i32,
    pub raw: String,
}

#[derive(Debug)]
pub enum RawmadError {
    SerializationError(Box<dyn std::error::Error>),
    DecodingMessage(Box<dyn std::error::Error>),
    Ws(tungstenite::Error),
    SendingMessage(tungstenite::Error)
}

impl TryInto<tungstenite::Message> for Message {
    type Error = RawmadError;
    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        let result = bincode::serialize(&self).map_err(|err| RawmadError::SerializationError(err))?; 
        Ok(tungstenite::Message::binary(result))
    }
}

impl TryFrom<tungstenite::Message> for Message {
    type Error = RawmadError;

    fn try_from(value: tungstenite::Message) -> Result<Self, Self::Error> {
        let bytes = value.into_data();
        let decoded_message = bincode::deserialize::<Message>(&bytes).map_err(|err| RawmadError::DecodingMessage(err))?;
        return Ok(decoded_message);
    }
}