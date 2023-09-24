use crate::chat;
use anyhow::Error;
use async_trait::async_trait;
use openai_rust::{futures_util::stream::BoxStream};

#[async_trait]
pub trait ApiCreation {
    fn new(api_key: &str) -> Self;
}

#[async_trait]
pub trait ApiRequest {
    // fn new(&self, api_key: &str) -> Self;
    async fn request(&self, messages: &Vec<chat::Message>);

   async fn create_chat_stream(
        &self,
        messages: &Vec<chat::Message>,
    ) -> Result<BoxStream<Result<String, Error>>, Error>;
}
