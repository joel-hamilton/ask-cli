use crate::state::AppMode;
use crate::ui::ui;
use crate::{
    state::{ChatState, AppModeState},
    traits::api_client::ApiRequest,
};
use anyhow::Error;
use crossterm::event::{KeyEvent};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    // style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::EnterAlternateScreen,
};
use ratatui::prelude::*;

// use inquire::{error::InquireResult, Editor, Text};
use std::io::{self};
pub struct App {
    pub api_client: Box<dyn ApiRequest>,
    pub chat_state: ChatState,
    app_mode_state: AppModeState,
}

impl App {
    pub fn new(
        api_client: Box<dyn ApiRequest>,
        chat_state: ChatState,
        app_mode_state: AppModeState,
    ) -> Self {
        App {
            api_client,
            chat_state,
            app_mode_state,
        }
    }

    pub fn get_terminal(&self) -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub async fn run(&mut self) -> io::Result<()> {
        // we will need to re-create term after returning from spawned editor process
        let mut terminal = self.get_terminal().unwrap();
        loop {
            terminal.draw(|f| ui(f, &mut self.chat_state,  &self.app_mode_state))?;

            if let Event::Key(key) = event::read()? {
                match self.app_mode_state.app_mode {
                    AppMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            let user_input = edit::edit("")?;

                            // restore our app's ui
                            let mut stdout = io::stdout();
                            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                            terminal = self.get_terminal().unwrap();
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &self.app_mode_state))?;

                            // update user message
                            self.chat_state.get_current_chat().push("user", &user_input);
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &self.app_mode_state))?;

                            // make request and update messages again
                            let _message = self
                                .api_client
                                .request(self.chat_state.get_current_chat().get_messages())
                                .await;
                            // self.chat_state
                            //     .get_current_chat()
                            //     .push("assistant", &message.content);
                            // terminal
                            //     .draw(|f| ui(f, &mut self.chat_state, &self.textarea_state, &self.app_mode_state))?;
                        }
                        KeyCode::Char('i') => {
                            self.app_mode_state.app_mode = AppMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    AppMode::Editing if key.kind == KeyEventKind::Press => match key {
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => {
                            // update messages and redraw immediately
                            self.chat_state
                                .get_current_chat()
                                .push("user", "");
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &self.app_mode_state))?;

                            // make request and update messages again
                            // let message = self
                            //     .api_client
                            //     .request(self.chat_state.get_current_chat().get_messages())
                            //     .await;
                            // self.chat_state
                            //     .get_current_chat()
                            //     .push("assistant", &message.content);

                            self.chat_state.get_current_chat().push("assistant", "temp");
                        }
                        KeyEvent {
                            code: KeyCode::Char(_to_insert),
                            ..
                        } => {
                        }
                        KeyEvent {
                            code: KeyCode::Backspace,
                            ..
                        } => {
                        }
                        KeyEvent {
                            code: KeyCode::Left,
                            ..
                        } => {
                        }
                        KeyEvent {
                            code: KeyCode::Right,
                            ..
                        } => {
                        }
                        KeyEvent {
                            code: KeyCode::Esc, ..
                        } => {
                            self.app_mode_state.app_mode = AppMode::Normal;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
