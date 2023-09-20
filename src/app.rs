use crate::input::InputMode;
use crate::ui::ui;
use crate::{
    state::{ChatState, InputState},
    traits::api_client::ApiRequest,
};
use anyhow::Error;
use crossterm::event::{KeyEvent, KeyModifiers};
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
    api_client: Box<dyn ApiRequest>,
    chat_state: ChatState,
    input_state: InputState,
}

impl App {
    pub fn new(api_client: Box<dyn ApiRequest>, chat_state: ChatState, input_state: InputState) -> Self {
        App {
            api_client,
            chat_state,
            input_state,
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
            terminal.draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;

            if let Event::Key(key) = event::read()? {
                match self.input_state.input.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            let user_input = edit::edit(&self.input_state.input.value)?;

                            // restore our app's ui
                            let mut stdout = io::stdout();
                            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                            terminal = self.get_terminal().unwrap();
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;

                            // update user message
                            self.chat_state.get_chat().push("user", &user_input);
                            self.input_state.input.clear();
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;

                            // make request and update messages again
                            let message = self
                                .api_client
                                .request(self.chat_state.get_chat().get_messages())
                                .await;
                            self.chat_state
                                .get_chat()
                                .push("assistant", &message.content);
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;
                        }
                        KeyCode::Char('i') => {
                            self.input_state.input.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key {
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => {
                            // update messages and redraw immediately
                            self.chat_state
                                .get_chat()
                                .push("user", &self.input_state.input.value);
                            self.input_state.input.clear();
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;

                            // make request and update messages again
                            let message = self
                                .api_client
                                .request(self.chat_state.get_chat().get_messages())
                                .await;
                            self.chat_state
                                .get_chat()
                                .push("assistant", &message.content);
                        }
                        KeyEvent {
                            code: KeyCode::Char(to_insert),
                            ..
                        } => {
                            self.input_state.input.enter_char(to_insert);
                        }
                        KeyEvent {
                            code: KeyCode::Backspace,
                            ..
                        } => {
                            self.input_state.input.delete_char();
                        }
                        KeyEvent {
                            code: KeyCode::Left,
                            ..
                        } => {
                            self.input_state.input.move_cursor_left();
                        }
                        KeyEvent {
                            code: KeyCode::Right,
                            ..
                        } => {
                            self.input_state.input.move_cursor_right();
                        }
                        KeyEvent {
                            code: KeyCode::Esc, ..
                        } => {
                            self.input_state.input.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
