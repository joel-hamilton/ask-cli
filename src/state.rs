use crate::chat::Chat;


pub struct State {
    chat_history: Vec<Chat>,
    current_chat_index: usize
}

impl State {
  pub fn default() -> State {
    State {
      chat_history: vec![Chat::default()],
      current_chat_index: 0
    }
  }

  pub fn get_chat(&mut self) -> &mut Chat {
    &mut self.chat_history[self.current_chat_index]
  }
}