use chrono::Utc;
use chrono_tz::Tz;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Clock ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let now = Utc::now();

    let zones: &[(&str, Tz)] = &[
        ("UTC", chrono_tz::UTC),
        ("NY ", chrono_tz::America::New_York),
        ("LON", chrono_tz::Europe::London),
        ("TYO", chrono_tz::Asia::Tokyo),
    ];

    let mut lines: Vec<Line> = Vec::new();

    for (name, tz) in zones {
        let local = now.with_timezone(tz);
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {} ", name),
                Style::default().fg(theme.dim),
            ),
            Span::styled(
                local.format("%H:%M").to_string(),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" {}", local.format("%Z")),
                Style::default().fg(theme.dim),
            ),
        ]));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
