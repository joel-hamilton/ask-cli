mod api;
mod apis;
mod app;
mod chat;
mod state;
mod traits;
mod ui;

use anyhow::Error;

use api::ApiClient;
use app::App;

use crossterm::{
    cursor::MoveToPreviousLine,
    execute,
    terminal::{Clear, ClearType},
};
use futures::StreamExt;
use inquire::Text;
use state::ChatState;
use std::io::stdout;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = ApiClient::new(&key, api::ClientType::Openai);
    let chat_state = ChatState::default();
    let mut app = App::new(api_client, chat_state);

    loop {
        let mut content = Text::new("Prompt:").prompt()?;

        if content == "history" {
            // TODO add history tui screen
            _ = app.run().await;
            continue;
        }

        if content.is_empty() {
            content = edit::edit("")?;
            _ = execute!(
                stdout(),
                MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            );

            print!("Prompt: {}", content);
        }

        app.chat_state.get_current_chat().push("user", &content);
        let mut complete_response: String = "".to_owned();
        match app
            .api_client
            .create_chat_stream(app.chat_state.get_current_chat().get_messages())
            .await
        {
            Ok(mut stream) => {
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(message) => {
                            complete_response += &message;
                            print!("{}", message);
                        }
                        Err(_e) => {}
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        println!();

        app.chat_state
            .get_current_chat()
            .get_messages()
            .push(chat::Message {
                role: "assistant".to_owned(),
                content: complete_response,
            });
    }
}
