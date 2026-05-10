use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::app::theme::{Theme, ALL_THEMES};
use crate::app::app_state::Modal;

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Profile ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(vec![
        Span::styled(" Username: ", Style::default().fg(theme.dim)),
        Span::styled(
            app.app_state.user.username.clone(),
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(vec![
        Span::styled(" Theme:    ", Style::default().fg(theme.dim)),
        Span::styled(
            app.app_state.user.theme_id.clone(),
            Style::default().fg(theme.accent),
        ),
        Span::styled(" [t]change", Style::default().fg(theme.dim)),
    ]));

    lines.push(Line::from(vec![
        Span::styled(" Timezone: ", Style::default().fg(theme.dim)),
        Span::styled(
            app.app_state.user.timezone.clone(),
            Style::default().fg(theme.fg),
        ),
    ]));

    lines.push(Line::from(Span::raw("")));

    lines.push(Line::from(Span::styled(
        " Bio:",
        Style::default().fg(theme.dim),
    )));

    if app.app_state.bio_editing {
        lines.push(Line::from(Span::styled(
            format!("  {}_", app.app_state.bio_buf),
            Style::default().fg(theme.fg),
        )));
        lines.push(Line::from(Span::styled(
            "  [Enter] save  [ESC] cancel",
            Style::default().fg(theme.dim),
        )));
    } else {
        let bio = app.app_state.user.bio.as_deref().unwrap_or("(no bio set)");
        lines.push(Line::from(Span::styled(
            format!("  {}", bio),
            Style::default().fg(theme.fg),
        )));
        lines.push(Line::from(Span::styled(
            "  [b]edit bio",
            Style::default().fg(theme.dim),
        )));
    }

    lines.push(Line::from(Span::raw("")));

    if let Some(cat) = &app.app_state.cat {
        let stage = cat.stage();
        let mood = cat.mood();
        let alive = if cat.is_alive { "alive" } else { "died" };
        lines.push(Line::from(vec![
            Span::styled(" Cat: ", Style::default().fg(theme.dim)),
            Span::styled(
                format!("stage {} ({}) — {} — {} growth pts", stage, alive, mood.as_str(), cat.growth_points),
                Style::default().fg(theme.secondary),
            ),
        ]));
    }

    // Key hints
    lines.push(Line::from(Span::raw("")));
    lines.push(Line::from(Span::styled(
        " [t]theme [b]bio [q]back",
        Style::default().fg(theme.dim),
    )));

    let para = Paragraph::new(lines)
        .style(Style::default().bg(theme.bg))
        .wrap(Wrap { trim: false });

    f.render_widget(para, inner);
}

pub fn draw_theme_picker(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Theme Picker (j/k to navigate, Enter to select) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let current = app.app_state.theme_picker_idx;
    let visible_height = inner.height as usize;

    // Scroll to keep current visible
    let start = if current >= visible_height {
        current - visible_height + 1
    } else {
        0
    };

    let items: Vec<ListItem> = ALL_THEMES
        .iter()
        .enumerate()
        .skip(start)
        .take(visible_height)
        .map(|(i, t)| {
            let selected = i == current;
            let prefix = if selected { "▶ " } else { "  " };
            // Color swatch block
            let swatch = "██";
            let style = if selected {
                Style::default()
                    .fg(t.primary)
                    .bg(t.bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(t.fg).bg(theme.bg)
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix.to_string(), style),
                Span::styled(swatch.to_string(), Style::default().fg(t.primary).bg(t.bg)),
                Span::styled(" ", style),
                Span::styled(t.name.to_string(), style),
            ]))
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner);
}
