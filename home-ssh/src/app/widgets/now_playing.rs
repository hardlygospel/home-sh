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
        .title(" Now Playing ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Synchronous access to now_playing
    let np = match app.state.now_playing.try_lock() {
        Ok(guard) => guard.clone(),
        Err(_) => None,
    };

    let mut lines: Vec<Line> = Vec::new();

    match np {
        Some(np) => {
            // Truncate artist/title to fit
            let max_w = inner.width.saturating_sub(2) as usize;

            let artist = if np.artist.len() > max_w {
                format!("{}…", &np.artist[..max_w.saturating_sub(1)])
            } else {
                np.artist.clone()
            };
            let title = if np.title.len() > max_w {
                format!("{}…", &np.title[..max_w.saturating_sub(1)])
            } else {
                np.title.clone()
            };

            lines.push(Line::from(Span::styled(
                format!(" {}", artist),
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                format!(" {}", title),
                Style::default().fg(theme.fg),
            )));

            // Progress bar (fake animated)
            let bar_width = inner.width.saturating_sub(4) as usize;
            let progress = if np.duration_secs > 0 {
                np.elapsed_secs as f32 / np.duration_secs as f32
            } else {
                0.5 // Unknown duration: show half
            };
            let filled = (progress * bar_width as f32) as usize;
            let bar = format!(
                " {}{}",
                "█".repeat(filled.min(bar_width)),
                "░".repeat(bar_width.saturating_sub(filled))
            );
            lines.push(Line::from(Span::styled(
                bar,
                Style::default().fg(theme.primary),
            )));

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" ♪ {} listeners", np.listeners),
                    Style::default().fg(theme.dim),
                ),
            ]));
        }
        None => {
            lines.push(Line::from(Span::styled(
                " No stream active",
                Style::default().fg(theme.dim),
            )));
            lines.push(Line::from(Span::raw("")));
            lines.push(Line::from(Span::styled(
                " ─────────────────",
                Style::default().fg(theme.border),
            )));
        }
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
