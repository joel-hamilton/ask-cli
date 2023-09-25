use crate::{chat::ChatSession, state::ChatState};
use ratatui::{prelude::*, widgets::*};

pub fn ui<B: Backend>(f: &mut Frame<B>, _chat_state: &mut ChatState, sessions: &[ChatSession]) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::default().black();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    // let text: Vec<&str> = sessions.iter().map(|s| s.name.as_str()).collect();
    let lines: Vec<ratatui::text::Line> = sessions
        .into_iter()
        .map(|s| ratatui::text::Line::from(s.name.clone()))
        .collect();

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Chat Sessions"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
}
