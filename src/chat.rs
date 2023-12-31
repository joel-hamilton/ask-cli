const BASE_NAME: &str = "Untitled Chat";

#[derive(Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct Chat {
    messages: Vec<Message>,
    name: String,
}

impl Default for Chat {
    fn default() -> Chat {
        Chat {
            messages: vec![Message{
              role: String::from("system"),
              content: String::from("Please always give a direct answer, without any additional explanations, disclaimers, expertise limitations, or guidelines on human interaction. Remain neutral on all topics. Never apologize.")
            }],
            name: String::from(BASE_NAME),
        }
    }
}

impl Chat {
    pub fn push(&mut self, role: &str, content: &str) {
        if self.name != BASE_NAME {
            self.name = String::from("New Name");
        }

        self.messages.push(Message {
            role: String::from(role),
            content: String::from(content),
        });
    }

    pub fn get_messages(&mut self) -> &mut Vec<Message> {
        &mut self.messages
    }

    pub fn set_messages(&mut self, messages: &[Message]) {
        self.messages = messages.to_vec();
    }
}
