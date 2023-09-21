use crate::{chat::Chat, input::Input};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct InputModeState {
    pub input_mode: InputMode,
}

impl InputModeState {
    pub fn default() -> Self {
        InputModeState {
            input_mode: InputMode::Editing,
        }
    }
}

enum Windows {
    INPUT,
    HISTORY,
}

pub struct WindowState {
    focused_window: Windows,
}

pub struct InputState {
    pub input: Input,
}

impl InputState {
    pub fn default() -> InputState {
        InputState {
            input: Input::default(),
        }
    }

    // pub fn get_input(&mut self) -> &mut Input {
    //   &mut self.input
    // }
}

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

    pub fn get_chat(&mut self) -> &mut Chat {
        &mut self.chat_history[self.current_chat_index]
    }
}
