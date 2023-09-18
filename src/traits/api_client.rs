use crate::chat;
use async_trait::async_trait;

#[async_trait]
pub trait Api {
    fn new(api_key: &str) -> Self;
    async fn request(&self, messages: &Vec<chat::Message>) -> Vec<chat::Message>;
}
