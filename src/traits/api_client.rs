use crate::chat;
use async_trait::async_trait;

#[async_trait]
pub trait ApiCreation {
    fn new( api_key: &str) -> Self;
}

#[async_trait]
pub trait ApiRequest {
    // fn new(&self, api_key: &str) -> Self;
    async fn request(&self, messages: &Vec<chat::Message>) -> chat::Message;
}
