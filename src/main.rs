mod api;
mod apis;
mod chat;
mod traits;
use crate::traits::api_client::Api;

use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    execute,
    style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use inquire::{error::InquireResult, Editor, Text};
use std::io::stdout;

#[tokio::main]
async fn main() -> InquireResult<()> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = api::ApiClient::new(&key, api::ClientType::OPENAI);
    let mut chat = chat::Chat::default();

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

        chat.push("user", &content);
        let messages = api_client.request(&chat.get_messages()).await;
        chat.set_messages(&messages);

        _ = stdout().execute(PrintStyledContent(
            "Response: "
                .with(Color::DarkMagenta)
                .attribute(Attribute::Bold),
        ));

        println!("{}", messages[messages.len() - 1].content)
    }
}
