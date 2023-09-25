use crate::chat::ChatSession;
use crate::db::DB;
use crate::ui::ui;
use crate::{state::ChatState, traits::api_client::ApiRequest};
use anyhow::Error;
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::EnterAlternateScreen,
};
use ratatui::prelude::*;

use std::io::{self};
pub struct App {
    pub api_client: Box<dyn ApiRequest>,
    pub chat_state: ChatState,
}

impl App {
    pub fn new(api_client: Box<dyn ApiRequest>, chat_state: ChatState) -> Self {
        App {
            api_client,
            chat_state,
        }
    }

    pub fn get_terminal(&self) -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub async fn run(&mut self, sessions: &[ChatSession]) -> anyhow::Result<i64> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        // we will need to re-create term after returning from spawned editor process
        let mut terminal = self.get_terminal().unwrap();
        loop {
            terminal.draw(|f| ui(f, &mut self.chat_state, sessions))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        return Ok(-1);
                    }
                    KeyCode::Char('1') => {
                        // TODO clear this screen and disable raw mode
                        return Ok(1);
                    }
                    _ => {}
                }
            }
        }
    }
}
