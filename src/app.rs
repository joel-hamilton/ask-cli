use crate::input::{Input, InputMode};
use crate::ui::ui;
use crate::{
    api::ApiClient,
    chat::Chat,
    state::{ChatState, InputState},
    traits::api_client::Api,
};
use anyhow::Error;
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::{
    cursor::{MoveTo, MoveToPreviousLine},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    // style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{prelude::*, widgets::*};

// use inquire::{error::InquireResult, Editor, Text};
use std::io::{self, stdout};
pub struct App {
    api_client: Box<dyn Api>,
    chat_state: ChatState,
    input_state: InputState,
}

impl App {
    pub fn new(api_client: Box<dyn Api>, chat_state: ChatState, input_state: InputState) -> Self {
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
                            self.input_state.input.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key {
                        KeyEvent {
                            code: KeyCode::Char('e'),
                            modifiers: KeyModifiers::CONTROL,
                            ..
                        } => {
                            let template = "Fill in the blank: Hello, _____!";
                            let edited = edit::edit(template)?;
                            let mut stdout = io::stdout();
                            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                            terminal = self.get_terminal().unwrap();
                            terminal
                                .draw(|f| ui(f, &mut self.chat_state, &mut self.input_state))?;
                        }
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
                            // let message = api_client
                            //     .request(chat_state.get_chat().get_messages())
                            //     .await;
                            // chat_state.get_chat().push("assistant", &message.content);
                            self.chat_state.get_chat().push("assistant", "testing");
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
