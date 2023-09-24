use crate::chat::Chat;

pub struct ChatState {
    chat_history: Vec<Chat>,
    current_chat_index: usize,
}

impl ChatState {
    pub fn default() -> ChatState {
        ChatState {
            chat_history: vec![Chat::default()],
            current_chat_index: 0,
        }
    }

    pub fn get_current_chat(&mut self) -> &mut Chat {
        &mut self.chat_history[self.current_chat_index]
    }
}