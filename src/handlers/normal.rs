use crate::{
    api::ApiClient,
    traits::api_client::{self, ApiRequest},
    utils,
};
use crossterm::{
    event::{EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::EnterAlternateScreen,
};
use std::io;

use crate::{
    state::{AppMode, AppModeState, ChatState, TextareaState},
    ui::ui,
};

pub struct NormalHandler {}

impl NormalHandler {
    pub fn default() -> Self {
        NormalHandler {}
    }
    pub async fn handle_event(
        &self,
        key: KeyEvent,
        api_client: &Box<(dyn ApiRequest + 'static)>,
        mut chat_state: &mut ChatState,
        mut textarea_state: &mut TextareaState,
        app_mode_state: &mut AppModeState,
    ) {
        match key.code {
            KeyCode::Char('e') => {
                let user_input = edit::edit(textarea_state.textarea.value.to_owned()).unwrap();

                // restore our app's ui
                let mut stdout = io::stdout();
                execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
                let mut terminal = utils::get_terminal().unwrap();
                terminal.draw(|f| ui(f, &mut chat_state, textarea_state, app_mode_state)).unwrap();

                // update user message
                chat_state.get_current_chat().push("user", &user_input);
                textarea_state.textarea.clear();
                terminal.draw(|f| ui(f, &mut chat_state, textarea_state, app_mode_state)).unwrap();

                // make request and update messages again
                let message = api_client
                    .request(chat_state.get_current_chat().get_messages())
                    .await;
                chat_state
                    .get_current_chat()
                    .push("assistant", &message.content);
                terminal.draw(|f| ui(f, &mut chat_state, textarea_state, app_mode_state)).unwrap();
            }
            KeyCode::Char('i') => {
                app_mode_state.app_mode = AppMode::Editing;
            }
            KeyCode::Char('q') => {
              panic!("exiting");
            }
            _ => {}
        }
    }
}
