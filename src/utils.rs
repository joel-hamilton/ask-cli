use std::io;

use crate::handlers::normal::NormalHandler;
use crate::state::AppMode;
use crate::ui::ui;
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

pub fn get_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    Ok(terminal)
}
