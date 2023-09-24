use crate::apis::openai::OpenaiClient;
use crate::traits::api_client::{ApiRequest, ApiCreation};

pub enum ClientType {
    Openai,
}

pub struct ApiClient {}

impl ApiClient {
    pub fn new(api_key: &str, client_type: ClientType) -> Box<dyn ApiRequest> {
        match client_type {
            ClientType::Openai => Box::new(OpenaiClient::new(api_key)),
        }
    }
}
