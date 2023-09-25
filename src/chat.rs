#[derive(Clone, Debug)]
pub struct ChatSession {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
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

    pub fn get_messages(&mut self) -> &mut Vec<Message> {
        &mut self.messages
    }
}
