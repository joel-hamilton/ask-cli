mod api;
mod apis;
mod app;
mod chat;
mod input;
mod state;
mod traits;
mod ui;

use anyhow::Error;

use api::ApiClient;
use app::App;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    // style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::{ChatState, InputState, InputModeState};
use std::io::{self};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = ApiClient::new(&key, api::ClientType::OPENAI);
    let chat_state = ChatState::default();
    let input_state = InputState::default();
    let input_mode_state = InputModeState::default();
    let mut app = App::new(api_client, chat_state, input_state, input_mode_state);

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
