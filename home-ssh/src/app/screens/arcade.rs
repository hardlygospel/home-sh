use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::app_state::ArcadeGame;
use crate::app::theme::Theme;

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    match app.app_state.arcade_game {
        ArcadeGame::Menu => draw_menu(f, app, theme, area),
        ArcadeGame::Snake => super::snake::draw(f, app, theme, area),
        ArcadeGame::Wordle => super::wordle::draw(f, app, theme, area),
    }
}

fn draw_menu(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Arcade ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(Span::raw("")),
        Line::from(Span::styled(
            "  Choose a game:",
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::raw("")),
        Line::from(vec![
            Span::styled("  [s] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Snake", Style::default().fg(theme.fg)),
            Span::styled("  —  hjkl or arrow keys, eat apples, don't die", Style::default().fg(theme.dim)),
        ]),
        Line::from(Span::raw("")),
        Line::from(vec![
            Span::styled("  [w] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            Span::styled("Wordle", Style::default().fg(theme.fg)),
            Span::styled("  —  guess the 5-letter word in 6 tries", Style::default().fg(theme.dim)),
        ]),
        Line::from(Span::raw("")),
        Line::from(Span::styled("  [q] back", Style::default().fg(theme.dim))),
    ];

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
