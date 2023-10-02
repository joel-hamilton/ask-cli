use crate::{state::ChatState, traits::api_client::ApiRequest};
use anyhow::Error;
use crossterm::event::{DisableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    // style::{Attribute, Color, PrintStyledContent, Stylize},
    terminal::EnterAlternateScreen,
};

// use inquire::{error::InquireResult, Editor, Text};
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
}
