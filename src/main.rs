use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    execute,
    style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use inquire::{error::InquireResult, Editor, Text};
use openai_rust::{chat, futures_util::StreamExt, Client};
use std::io::stdout;

#[tokio::main]
async fn main() -> InquireResult<()> {
    let client = Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let mut messages: Vec<chat::Message> = Vec::new();

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

        messages.push(chat::Message {
            role: "user".to_owned(),
            content: content,
        });

        let args = chat::ChatArguments::new("gpt-4", messages.clone());
        let mut res = client.create_chat_stream(args).await.unwrap();

        _ = stdout().execute(PrintStyledContent(
            "Response: "
                .with(Color::DarkMagenta)
                .attribute(Attribute::Bold),
        ));

        let mut complete_response: String = "".to_owned();
        while let Some(events) = res.next().await {
            for event in events.unwrap() {
                complete_response += &event.to_string();
                print!("{}", event)
            }
        }

        println!("");

        messages.push(chat::Message {
            role: "assistant".to_owned(),
            content: complete_response,
        });
    }
}
