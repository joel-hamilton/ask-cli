use crate::chat::Chat;

pub struct ChatState {
    pub current_chat: Chat,
    pub current_session_id: Option<i64>,
}

impl ChatState {
    pub fn default() -> ChatState {
        ChatState {
            current_chat: Chat::default(),
            current_session_id: None,
        }
    }
}
