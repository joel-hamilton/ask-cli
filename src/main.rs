mod api;
mod apis;
mod chat;
mod input;
mod state;
mod traits;
mod ui;

use crate::input::{Input, InputMode};
use crate::traits::api_client::Api;
use crate::ui::ui;

use anyhow::Error;
use api::ApiClient;
use chat::Chat;
use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    // style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use state::{ChatState, InputState};
use std::char::CharTryFromError;
// use inquire::{error::InquireResult, Editor, Text};
use std::io::{self, stdout};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let mut api_client = api::ApiClient::new(&key, api::ClientType::OPENAI);
    let mut chat_state = ChatState::default();
    let mut input_state = InputState::default();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(
        &mut api_client,
        &mut terminal,
        &mut chat_state,
        &mut input_state,
    )
    .await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend, A: Api + 'static>(
    api_client: &mut A,
    terminal: &mut Terminal<B>,
    chat_state: &mut ChatState,
    input_state: &mut InputState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, chat_state, input_state))?;

        if let Event::Key(key) = event::read()? {
            match input_state.input.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        input_state.input.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        // update messages and redraw immediately
                        chat_state.get_chat().push("user", &input_state.input.value);
                        input_state.input.clear();
                        terminal.draw(|f| ui(f, chat_state, input_state))?;

                        // make request and update messages again
                        let message = api_client
                            .request(chat_state.get_chat().get_messages())
                            .await;

                        chat_state.get_chat().push("assistant", &message.content);
                    }
                    KeyCode::Char(to_insert) => {
                        input_state.input.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        input_state.input.delete_char();
                    }
                    KeyCode::Left => {
                        input_state.input.move_cursor_left();
                    }
                    KeyCode::Right => {
                        input_state.input.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        input_state.input.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
