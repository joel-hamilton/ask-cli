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
use crossterm::event::{KeyEvent, KeyModifiers};
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
use std::collections::HashMap;
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
    let mut terminal = get_terminal().unwrap();
        
    // create app and run it
    let res = run_app(
        &mut api_client,
        // &mut terminal,
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

fn get_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    Ok(terminal)
}

async fn run_app<A: Api + 'static>(
    api_client: &mut A,
    chat_state: &mut ChatState,
    input_state: &mut InputState,
) -> io::Result<()> {
    // we will need to re-create term after returning from spawned editor process
    let mut terminal = get_terminal().unwrap();
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
                InputMode::Editing if key.kind == KeyEventKind::Press => match key {
                    KeyEvent {
                        code: KeyCode::Char('e'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        let template = "Fill in the blank: Hello, _____!";
                        let edited = edit::edit(template)?;
                        // println!("after editing: '{}'", edited);
                        let mut stdout = io::stdout();
                        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                        terminal = get_terminal().unwrap();
                        terminal.draw(|f| ui(f, chat_state, input_state))?;
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        // update messages and redraw immediately
                        chat_state.get_chat().push("user", &input_state.input.value);
                        input_state.input.clear();
                        terminal.draw(|f| ui(f, chat_state, input_state))?;

                        // make request and update messages again
                        // let message = api_client
                        //     .request(chat_state.get_chat().get_messages())
                        //     .await;
                        // chat_state.get_chat().push("assistant", &message.content);
                        chat_state.get_chat().push("assistant", "testing");
                    }
                    KeyEvent {
                        code: KeyCode::Char(to_insert),
                        ..
                    } => {
                        input_state.input.enter_char(to_insert);
                    }
                    KeyEvent {
                        code: KeyCode::Backspace,
                        ..
                    } => {
                        input_state.input.delete_char();
                    }
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => {
                        input_state.input.move_cursor_left();
                    }
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => {
                        input_state.input.move_cursor_right();
                    }
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => {
                        input_state.input.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
