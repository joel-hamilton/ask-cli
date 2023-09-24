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
    terminal::{
        Clear, ClearType,
    },
};
use futures::StreamExt;
use inquire::Text;
use state::{AppModeState, ChatState};
use std::io::stdout;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = std::env::var("OPENAI_API_KEY").unwrap();
    let api_client = ApiClient::new(&key, api::ClientType::OPENAI);
    let chat_state = ChatState::default();
    let app_mode_state = AppModeState::default();
    let mut app = App::new(api_client, chat_state, app_mode_state);

    loop {
        let mut content = Text::new("Prompt:").prompt()?;
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

    // // setup terminal
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // let res = app.run().await;

    // let mut terminal = app.get_terminal().unwrap();
    // // create app and run it
    // // let res = run_app(&mut api_client, &mut chat_state, &mut textarea_state).await;

    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    // if let Err(err) = res {
    //     println!("{err:?}");
    // }

    Ok(())
}
