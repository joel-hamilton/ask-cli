use crate::apis::openai::OpenaiClient;
use crate::traits::api_client::Api;
use crate::chat;

use crossterm::cursor::RestorePosition;
// use openai_rust::{
//     chat::{self as openai_chat, stream::ChatResponseEvent},
//     futures_util::{Stream, StreamExt},
//     Client,
// };

pub enum ClientType {
    OPENAI,
}

pub struct ApiClient {
    client: Box<dyn Api>,
}

impl ApiClient {
    pub fn new(api_key: &str, client_type: ClientType) -> OpenaiClient {
        match client_type {
            ClientType::OPENAI => OpenaiClient::new(api_key),
        }
    }
}
