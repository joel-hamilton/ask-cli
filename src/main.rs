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
use state::State;
// use inquire::{error::InquireResult, Editor, Text};
use std::io::{self, stdout};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = api::ApiClient::new(&key, api::ClientType::OPENAI);
    let mut state = state::State::default();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, &mut state);

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

    // let _ = execute!(
    //     stdout(),
    //     Clear(ClearType::All),
    //     Clear(ClearType::Purge),
    //     MoveTo(0, 0)
    // );

    // loop {
    //     let mut content = Text::new("Prompt:").prompt()?;

    //     if content == "" {
    //         _ = execute!(
    //             stdout(),
    //             MoveToPreviousLine(1),
    //             Clear(ClearType::CurrentLine)
    //         );

    //         content = Editor::new("Prompt:").prompt()?;

    //         _ = execute!(
    //             stdout(),
    //             MoveToPreviousLine(1),
    //             Clear(ClearType::CurrentLine)
    //         );

    //         println!("Prompt: {}", content);
    //     }

    //     if content == "" {
    //         _ = execute!(
    //             stdout(),
    //             MoveToPreviousLine(1),
    //             Clear(ClearType::CurrentLine)
    //         );
    //         continue;
    //     }

    //     state.get_chat().push("user", &content);
    //     let messages = api_client.request(&state.get_chat().get_messages()).await;
    //     state.get_chat().set_messages(&messages);

    //     _ = stdout().execute(PrintStyledContent(
    //         "Response: "
    //             .with(Color::DarkMagenta)
    //             .attribute(Attribute::Bold),
    //     ));

    //     println!(
    //         "{}",
    //         state.get_chat().get_messages()[messages.len() - 1].content
    //     )
    // }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        // let mut state_clone = state.get_clone();
        terminal.draw(|f| ui(f, state))?;

        let input = state.get_input();
        if let Event::Key(key) = event::read()? {
            match input.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        input.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        // state.get_chat().push("user", &content);
                        // let messages = api_client.request(&state.get_chat().get_messages()).await;
                        // state.get_chat().set_messages(&messages);
                        input.submit_message()
                    }
                    KeyCode::Char(to_insert) => {
                        input.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        input.delete_char();
                    }
                    KeyCode::Left => {
                        input.move_cursor_left();
                    }
                    KeyCode::Right => {
                        input.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        input.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
