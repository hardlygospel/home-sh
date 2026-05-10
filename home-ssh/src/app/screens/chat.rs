use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;
use crate::app::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(format!(" #{} ", app.app_state.current_room_name()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split: messages area + input
    let input_height = if app.app_state.input_mode { 3 } else { 1 };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(input_height),
        ])
        .split(inner);

    draw_messages(f, app, theme, chunks[0]);
    draw_input(f, app, theme, chunks[1]);
}

fn draw_messages(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let messages = &app.app_state.messages;

    if messages.is_empty() {
        let para = Paragraph::new(vec![
            Line::from(Span::styled(
                "  No messages yet. Press [i] to chat.",
                Style::default().fg(theme.dim),
            )),
        ])
        .style(Style::default().bg(theme.bg));
        f.render_widget(para, area);
        return;
    }

    let total = messages.len();
    let visible_height = area.height as usize;

    // Calculate which messages to show (latest at bottom)
    let start = if total > visible_height {
        total - visible_height
    } else {
        0
    };

    let items: Vec<ListItem> = messages[start..]
        .iter()
        .map(|msg| {
            let time = msg.created_at.format("%H:%M").to_string();
            let own = msg.user_id == app.app_state.user.id;

            let username_style = if own {
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD)
            };

            // Highlight @mentions
            let body_spans = highlight_mentions(&msg.body, &app.app_state.user.username, theme);

            let mut line_spans = vec![
                Span::styled(format!(" {} ", time), Style::default().fg(theme.dim)),
                Span::styled(format!("{}: ", msg.username), username_style),
            ];
            line_spans.extend(body_spans);

            ListItem::new(Line::from(line_spans))
        })
        .collect();

    let list = List::new(items).style(Style::default().bg(theme.bg));
    f.render_widget(list, area);
}

fn highlight_mentions<'a>(body: &'a str, own_username: &str, theme: &'a Theme) -> Vec<Span<'a>> {
    // Simple mention highlighting: find @username patterns
    let mut spans = Vec::new();
    let mention = format!("@{}", own_username);

    let mut remaining = body;
    while !remaining.is_empty() {
        if let Some(pos) = remaining.find('@') {
            if pos > 0 {
                spans.push(Span::raw(remaining[..pos].to_string()));
                remaining = &remaining[pos..];
            }
            // Check if it matches a mention
            let end = remaining[1..]
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .map(|p| p + 1)
                .unwrap_or(remaining.len());
            let word = &remaining[..end];
            if word == mention {
                spans.push(Span::styled(
                    word.to_string(),
                    Style::default().fg(theme.warning).add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    word.to_string(),
                    Style::default().fg(theme.accent),
                ));
            }
            remaining = &remaining[end..];
        } else {
            spans.push(Span::raw(remaining.to_string()));
            break;
        }
    }

    if spans.is_empty() {
        spans.push(Span::raw(body.to_string()));
    }
    spans
}

fn draw_input(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    if app.app_state.input_mode {
        let block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.primary));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let input_text = format!("> {}_", app.app_state.input_buf);
        let para = Paragraph::new(Line::from(Span::styled(
            input_text,
            Style::default().fg(theme.fg),
        )))
        .wrap(Wrap { trim: true });
        f.render_widget(para, inner);
    } else {
        let hint = Paragraph::new(Line::from(Span::styled(
            " Press [i] to type a message",
            Style::default().fg(theme.dim),
        )));
        f.render_widget(hint, area);
    }
}
