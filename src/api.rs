use crate::apis::openai::OpenaiClient;
use crate::traits::api_client::{ApiRequest, ApiCreation};

pub enum ClientType {
    OPENAI,
}

pub struct ApiClient {}

impl ApiClient {
    pub fn new(api_key: &str, client_type: ClientType) -> Box<dyn ApiRequest> {
        match client_type {
            ClientType::OPENAI => Box::new(OpenaiClient::new(api_key)),
        }
    }
}
