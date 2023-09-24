

use crate::chat;
use crate::traits::api_client::ApiRequest;
use crate::{chat::Message, traits::api_client::ApiCreation};
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::BoxStream;
use openai_rust::{chat as openai_chat, futures_util::StreamExt, Client};

pub struct OpenaiClient {
    client: Client,
}

#[async_trait]
impl ApiRequest for OpenaiClient {
    async fn request(&self, messages: &[Message]) {
        let messages = messages.clone();
        let chat_args = openai_chat::ChatArguments::new(
            "gpt-3.5-turbo",
            self.chat_messages_to_openai_messages(&messages),
        );

        let mut res = self.client.create_chat_stream(chat_args).await.unwrap();
        let mut assistant_message: String = String::new();

        while let Some(events) = res.next().await {
            for event in events.unwrap() {
                assistant_message += &event.to_string();
            }
        }
    }

    async fn create_chat_stream(
        &self,
        messages: &[chat::Message],
    ) -> Result<BoxStream<Result<String, Error>>, Error> {
        let chat_args = openai_chat::ChatArguments::new(
            "gpt-3.5-turbo",
            self.chat_messages_to_openai_messages(messages),
        );
        match self.client.create_chat_stream(chat_args).await {
            Ok(s) => {
                let transformed = s.map(|item| {
                    match item {
                        Ok(events) => {
                            let transformed_events = events
                                .into_iter()
                                .map(|event| event.to_string())
                                .collect::<Vec<_>>();

                            Ok(transformed_events.join(""))
                        }
                        Err(e) => Err(e), // Change error type if required
                    }
                });

                Ok(Box::pin(transformed))
            }
            Err(e) => Err(e),
        }
    }
}

impl ApiCreation for OpenaiClient {
    fn new(api_key: &str) -> OpenaiClient {
        let client = Client::new(api_key);
        OpenaiClient { client }
    }
}
impl OpenaiClient {
    fn chat_messages_to_openai_messages(
        &self,
        messages: &[Message],
    ) -> Vec<openai_chat::Message> {
        messages
            .iter()
            .map(|message: &Message| -> openai_chat::Message {
                openai_chat::Message {
                    role: message.role.to_string(),
                    content: message.content.to_string(),
                }
            })
            .collect()
    }
}
