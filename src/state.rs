use crate::{chat::Chat, input::Input};

pub struct State {
    input: Input,
    chat_history: Vec<Chat>,
    current_chat_index: usize,
}

impl State {
    pub fn default() -> State {
        State {
            input: Input::default(),
            chat_history: vec![Chat::default()],
            current_chat_index: 0,
        }
    }

    pub fn get_clone(&self) -> State {
      State {
        input: self.input.clone(),
        chat_history: self.chat_history.clone(),
        current_chat_index: self.current_chat_index
      }
    }

    pub fn get_chat(&mut self) -> &mut Chat {
        &mut self.chat_history[self.current_chat_index]
    }

    pub fn get_input(&mut self) -> &mut Input {
      &mut self.input
    }
}
