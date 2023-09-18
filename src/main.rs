mod api;
mod apis;
mod chat;
mod state;
mod traits;
use crate::traits::api_client::Api;

use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use inquire::{error::InquireResult, Editor, Text};
use std::io::stdout;

#[tokio::main]
async fn main() -> InquireResult<()> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = api::ApiClient::new(&key, api::ClientType::OPENAI);
    // let mut chat = chat::Chat::default();
    let mut state = state::State::default();

    let _ = execute!(
        stdout(),
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        MoveTo(0, 0)
    );

    loop {
        let mut content = Text::new("Prompt:").prompt()?;

        if content == "" {
            _ = execute!(
                stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            );

            content = Editor::new("Prompt:").prompt()?;

            _ = execute!(
                stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            );

            println!("Prompt: {}", content);
        }

        if content == "" {
            _ = execute!(
                stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            );
            continue;
        }

        state.get_chat().push("user", &content);
        let messages = api_client.request(&state.get_chat().get_messages()).await;
        state.get_chat().set_messages(&messages);

        _ = stdout().execute(PrintStyledContent(
            "Response: "
                .with(Color::DarkMagenta)
                .attribute(Attribute::Bold),
        ));

        println!(
            "{}",
            state.get_chat().get_messages()[messages.len() - 1].content
        )
    }
}
