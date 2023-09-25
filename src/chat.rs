use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct ChatSession {
    pub id: i64,
    pub name: String,
}

impl Display for ChatSession {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        let str = self.name.as_str();
        formatter.write_str(str);
        Ok(())
    }
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
    pub fn new(messages: &Vec<Message>) -> Self {
        Chat {
          messages: messages.to_vec()
        }
    }

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
