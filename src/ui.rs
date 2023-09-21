use crate::state::{AppMode, AppModeState, ChatState, TextareaState};
use ratatui::{prelude::*, widgets::*};

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    chat_state: &mut ChatState,
    textarea_state: &TextareaState,
    app_mode_state: &AppModeState,
) {
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

    let (msg, style) = match app_mode_state.app_mode {
        AppMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "i".bold(),
                " to edit, ".into(),
                "e".bold(),
                " to launch editor.".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        AppMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to exit edit mode, ".into(),
                "Enter".bold(),
                " send prompt".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let paragraph = Paragraph::new(textarea_state.textarea.value.as_str())
        .style(match app_mode_state.app_mode {
            AppMode::Normal => Style::default(),
            AppMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(paragraph, chunks[2]);

    match app_mode_state.app_mode {
        AppMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        AppMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the textarea
                // This position is can be controlled via the left and right arrow key
                chunks[2].x + textarea_state.textarea.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }

    let messages: Vec<ListItem> = chat_state
        .get_current_chat()
        .get_messages()
        .iter()
        .map(|message| {
            let content = Line::from(Span::raw(format!("{}: {}", message.role, message.content)));
            ListItem::new(content)
        })
        .collect();

    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[1]);

    let history = vec![
        ListItem::new("Session 1"),
        ListItem::new("Session 2"),
        ListItem::new("Session 3"),
        ListItem::new("Session 4"),
    ];
    let chat_history =
        List::new(history).block(Block::default().borders(Borders::ALL).title("Chat History"));
    f.render_widget(chat_history, cols[0]);
    f.render_widget(messages, cols[1]);
}
