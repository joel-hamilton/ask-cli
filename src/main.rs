mod api;
mod apis;
mod app;
mod chat;
mod db;
mod state;
mod traits;
mod ui;

use anyhow::Error;
use api::ApiClient;
use app::App;
use db::DB;

use crossterm::{
    cursor::MoveToPreviousLine,
    execute,
    terminal::{Clear, ClearType},
};
use inquire::{Select, Text};
use state::ChatState;
use std::io::stdout;

use crate::chat::Chat;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = ApiClient::new(&key, api::ClientType::Openai);
    let chat_state = ChatState::default();
    let mut app = App::new(api_client, chat_state);
    let db = DB::new().await;
    let _ = db.migrate().await;

    loop {
        let mut content = Text::new("Prompt:").prompt()?;

        if content == "history" {
            let sessions = db.get_sessions().await.unwrap();
            let session = Select::new("Load a chat session", sessions)
                .prompt()
                .unwrap();

            let session_messages = db.get_session(session.id).await.unwrap();
            let new_chat = Chat::new(&session_messages);
            app.chat_state.current_chat = new_chat;
            app.chat_state.current_session_id = Some(session.id);

            for message in session_messages {
                println!("{}: {}", message.role, message.content);
            }
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

        app.chat_state.current_chat.push("user", &content);
        let session_id = db
            .add_chat("user", &content, app.chat_state.current_session_id)
            .await
            .unwrap();
        if let None = app.chat_state.current_session_id {
            app.chat_state.current_session_id = Some(session_id);
        }

        let mut complete_response: String = "".to_owned();
        // match app
        //     .api_client
        //     .create_chat_stream(app.chat_state.current_chat.get_messages())
        //     .await
        // {
        //     Ok(mut stream) => {
        //         while let Some(item) = stream.next().await {
        //             match item {
        //                 Ok(message) => {
        //                     complete_response += &message;
        //                     print!("{}", message);
        //                 }
        //                 Err(_e) => {}
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // }

        // println!();

        complete_response = String::from("testing!");
        println!("{}", complete_response);

        app.chat_state
            .current_chat
            .get_messages()
            .push(chat::Message {
                role: "assistant".to_owned(),
                content: complete_response.to_owned(),
            });

        let _ = db
            .add_chat(
                "assistant",
                &complete_response,
                app.chat_state.current_session_id,
            )
            .await;
    }
}
