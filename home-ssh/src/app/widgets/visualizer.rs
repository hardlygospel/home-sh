use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::theme::Theme;

const VIZ_CHARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Viz ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Draw 8-band visualizer using block characters
    let bars = &app.app_state.viz_bars;
    let height = inner.height as usize;
    let bar_width = (inner.width as usize / 8).max(1);

    let mut lines: Vec<Line> = Vec::with_capacity(height);

    for row in 0..height {
        let threshold = 1.0 - (row as f32 / height as f32);
        let mut spans: Vec<Span> = Vec::new();

        for &bar in bars {
            let level = bar;
            let ch = if level >= threshold {
                // How much of this row is filled
                let overshoot = (level - threshold) * height as f32;
                let idx = (overshoot * 8.0).min(8.0) as usize;
                VIZ_CHARS[idx.min(VIZ_CHARS.len() - 1)]
            } else {
                ' '
            };

            let color = if level > 0.7 {
                theme.error
            } else if level > 0.4 {
                theme.warning
            } else {
                theme.primary
            };

            spans.push(Span::styled(
                ch.to_string().repeat(bar_width),
                Style::default().fg(color),
            ));
        }

        lines.push(Line::from(spans));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
