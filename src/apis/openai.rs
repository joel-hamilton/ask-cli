use crate::chat::{self, Message};
use crate::traits::api_client::Api;
use async_trait::async_trait;
use openai_rust::{chat as openai_chat, futures_util::StreamExt, Client};

pub struct OpenaiClient {
    client: Client,
}

#[async_trait]
impl Api for OpenaiClient {
    async fn request(&self, messages: &Vec<Message>) -> Message {
        let mut messages = messages.clone();
        let chat_args = openai_chat::ChatArguments::new(
            "gpt-4",
            self.chat_messages_to_openai_messages(&messages),
        );

        let mut res = self.client.create_chat_stream(chat_args).await.unwrap();
        let mut assistant_message = String::from("");
        // let mut complete_response: String = "".to_owned();
        while let Some(events) = res.next().await {
            for event in events.unwrap() {
                assistant_message += &event.to_string();
            }
        }

        let message = Message {
            role: String::from("assistant"),
            content: assistant_message,
        };
        let return_message = message.clone();
        messages.push(message);

        return_message
    }
}

impl OpenaiClient {
    pub fn new(api_key: &str) -> OpenaiClient {
        let client = Client::new(api_key);
        return OpenaiClient { client };
    }

    fn chat_messages_to_openai_messages(
        &self,
        messages: &Vec<Message>,
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
