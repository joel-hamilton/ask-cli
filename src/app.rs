use crate::handlers::normal::NormalHandler;
use crate::state::AppMode;
use crate::ui::ui;
use crate::utils;
use crate::{
    state::{AppModeState, ChatState, TextareaState},
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
    textarea_state: TextareaState,
    app_mode_state: AppModeState,
}

impl App {
    pub fn new(
        api_client: Box<dyn ApiRequest>,
        chat_state: ChatState,
        textarea_state: TextareaState,
        app_mode_state: AppModeState,
    ) -> Self {
        App {
            api_client,
            chat_state,
            textarea_state,
            app_mode_state,
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        let normal_handler = NormalHandler::default();

        // we will need to re-create term after returning from spawned editor process
        let mut terminal = utils::get_terminal().unwrap();
        loop {
            terminal.draw(|f| {
                ui(
                    f,
                    &mut self.chat_state,
                    &self.textarea_state,
                    &self.app_mode_state,
                )
            })?;

            if let Event::Key(key) = event::read()? {
                match self.app_mode_state.app_mode {
                    AppMode::Normal => {
                        normal_handler
                            .handle_event(
                                key,
                                &self.api_client,
                                &mut self.chat_state,
                                &mut self.textarea_state,
                                &mut self.app_mode_state,
                            )
                            .await
                    }
                    AppMode::Editing => {
                        normal_handler
                            .handle_event(
                                key,
                                &self.api_client,
                                &mut self.chat_state,
                                &mut self.textarea_state,
                                &mut self.app_mode_state,
                            )
                            .await
                    }
                }
            }

            // if let Event::Key(key) = event::read()? {
            //     match self.app_mode_state.app_mode {
            //         AppMode::Normal => "HI",
            //         AppMode::Editing if key.kind == KeyEventKind::Press => match key {
            //             KeyEvent {
            //                 code: KeyCode::Enter,
            //                 ..
            //             } => {
            //                 // update messages and redraw immediately
            //                 self.chat_state
            //                     .get_current_chat()
            //                     .push("user", &self.textarea_state.textarea.value);
            //                 self.textarea_state.textarea.clear();
            //                 terminal.draw(|f| {
            //                     ui(
            //                         f,
            //                         &mut self.chat_state,
            //                         &self.textarea_state,
            //                         &self.app_mode_state,
            //                     )
            //                 })?;

            //                 // make request and update messages again
            //                 // let message = self
            //                 //     .api_client
            //                 //     .request(self.chat_state.get_current_chat().get_messages())
            //                 //     .await;
            //                 // self.chat_state
            //                 //     .get_current_chat()
            //                 //     .push("assistant", &message.content);

            //                 self.chat_state.get_current_chat().push("assistant", "temp");
            //             }
            //             KeyEvent {
            //                 code: KeyCode::Char(to_insert),
            //                 ..
            //             } => {
            //                 self.textarea_state.textarea.enter_char(to_insert);
            //             }
            //             KeyEvent {
            //                 code: KeyCode::Backspace,
            //                 ..
            //             } => {
            //                 self.textarea_state.textarea.delete_char();
            //             }
            //             KeyEvent {
            //                 code: KeyCode::Left,
            //                 ..
            //             } => {
            //                 self.textarea_state.textarea.move_cursor_left();
            //             }
            //             KeyEvent {
            //                 code: KeyCode::Right,
            //                 ..
            //             } => {
            //                 self.textarea_state.textarea.move_cursor_right();
            //             }
            //             KeyEvent {
            //                 code: KeyCode::Esc, ..
            //             } => {
            //                 self.app_mode_state.app_mode = AppMode::Normal;
            //             }
            //             _ => {}
            //         },
            //         _ => {}
            //     }
            // }
        }
    }
}
