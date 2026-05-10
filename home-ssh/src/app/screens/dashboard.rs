use chrono::Utc;
use chrono_tz::Tz;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Dashboard ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // World clocks
            Constraint::Min(0),    // HN stories
        ])
        .split(inner);

    draw_clocks(f, app, theme, chunks[0]);
    draw_hn_stories(f, app, theme, chunks[1]);
}

fn draw_clocks(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let now_utc = Utc::now();

    let zones: &[(&str, Tz)] = &[
        ("UTC    ", chrono_tz::UTC),
        ("New York", chrono_tz::America::New_York),
        ("London ", chrono_tz::Europe::London),
        ("Tokyo  ", chrono_tz::Asia::Tokyo),
        ("Sydney ", chrono_tz::Australia::Sydney),
    ];

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        "── World Clocks ──",
        Style::default().fg(theme.dim).add_modifier(Modifier::BOLD),
    )));

    for (name, tz) in zones {
        let local = now_utc.with_timezone(tz);
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}: ", name),
                Style::default().fg(theme.dim),
            ),
            Span::styled(
                local.format("%H:%M").to_string(),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, area);
}

fn draw_hn_stories(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Hacker News Top Stories ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Get stories from shared state synchronously
    let stories = match app.state.hn_stories.try_lock() {
        Ok(guard) => guard.clone(),
        Err(_) => vec![],
    };

    if stories.is_empty() {
        let para = Paragraph::new(vec![
            Line::from(Span::styled(
                "  Loading HN stories...",
                Style::default().fg(theme.dim),
            )),
        ])
        .style(Style::default().bg(theme.bg));
        f.render_widget(para, inner);
        return;
    }

    let scroll = app.app_state.hn_scroll.min(stories.len().saturating_sub(1));
    let visible: Vec<ListItem> = stories
        .iter()
        .skip(scroll)
        .take(inner.height as usize)
        .enumerate()
        .map(|(i, article)| {
            let n = scroll + i + 1;
            let title = if article.title.len() > inner.width as usize - 10 {
                format!("{}...", &article.title[..inner.width as usize - 13])
            } else {
                article.title.clone()
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {:2}. ", n),
                    Style::default().fg(theme.dim),
                ),
                Span::styled(title, Style::default().fg(theme.fg)),
                Span::styled(
                    format!(" ↑{}", article.score),
                    Style::default().fg(theme.accent),
                ),
            ]))
        })
        .collect();

    let list = List::new(visible).style(Style::default().bg(theme.bg));
    f.render_widget(list, inner);
}
