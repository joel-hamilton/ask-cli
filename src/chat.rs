use openai_rust::{chat, futures_util::StreamExt, Client};

#[derive(Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct Chat {
    messages: Vec<Message>,
}

impl Default for Chat {
    fn default() -> Chat {
        Chat {
            messages: Vec::new(),
        }
    }
}

impl Chat {
    pub fn push(&mut self, role: &str, content: &str) {
        self.messages.push(Message {
            role: String::from(role),
            content: String::from(content),
        });
    }

    pub fn get_messages(&mut self) -> &Vec<Message> {
        &self.messages
    }

    pub fn set_messages(&mut self, messages: &Vec<Message>) {
        self.messages = messages.to_vec();
    }
}
