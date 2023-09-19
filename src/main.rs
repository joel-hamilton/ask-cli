mod api;
mod apis;
mod app;
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
use app::App;
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
};
use ratatui::{prelude::*, widgets::*};
use state::{ChatState, InputState};
// use inquire::{error::InquireResult, Editor, Text};
use std::io::{self, stdout};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let mut api_client: apis::openai::OpenaiClient =
        api::ApiClient::new(&key, api::ClientType::OPENAI);
    let mut chat_state = ChatState::default();
    let mut input_state = InputState::default();
    let mut app = App::new(Box::new(api_client), chat_state, input_state);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let res = app.run().await;

    let mut terminal = app.get_terminal().unwrap();
    // create app and run it
    // let res = run_app(&mut api_client, &mut chat_state, &mut input_state).await;

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
