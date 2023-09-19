use crate::input::{InputMode};
use crate::state::{ChatState, InputState};
use ratatui::{prelude::*, widgets::*};

pub fn ui<B: Backend>(f: &mut Frame<B>, chat_state: &mut ChatState, input_state: &mut InputState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    
        let (msg, style) = match input_state.input.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Line::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);

        let paragraph = Paragraph::new(input_state.input.value.as_str())
            .style(match input_state.input.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(paragraph, chunks[2]);

        match input_state.input.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask ratatui to put it at the specified coordinates after
                // rendering
                f.set_cursor(
                    // Draw the cursor at the current position in the input field.
                    // This position is can be controlled via the left and right arrow key
                    chunks[2].x + input_state.input.cursor_position as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[2].y + 1,
                )
            }
        }
    

    let messages: Vec<ListItem> = chat_state
        .get_chat()
        .get_messages()
        .iter()
        .map(|message| {
            let content = Line::from(Span::raw(format!("{}: {}", message.role, message.content)));
            ListItem::new(content)
        })
        .collect();

    // let messages: Vec<ListItem> = input
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = Line::from(Span::raw(format!("{i}: {m}")));
    //         ListItem::new(content)
    //     })
    //     .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[1]);
}
