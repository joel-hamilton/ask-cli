use crate::chat::;
use anyhow::Error;
use async_trait::async_trait;
use openai_rust::futures_util::stream::BoxStream;

#[async_trait]
pub trait ApiCreation {
    fn new(api_key: &str) -> Self;
}

#[async_trait]
pub trait ApiRequest {
    async fn request(&self, messages: &[Message]);

   async fn create_chat_stream(
        &self,
        messages: &[Message],
    ) -> Result<BoxStream<Result<String, Error>>, Error>;
}
