use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::app::app_state::{Modal, Screen};
use crate::app::theme::get_theme;
use crate::app::App;

pub async fn render(app: &mut App) -> Result<()> {
    let theme_id = app.app_state.user.theme_id.clone();
    let theme = get_theme(&theme_id);
    let w = app.width;
    let h = app.height;

    // Collect all render bytes in a Vec<u8> first (avoids borrow issues)
    let mut buf: Vec<u8> = Vec::with_capacity(8192);
    {
        let backend = CrosstermBackend::new(&mut buf);
        let mut terminal = Terminal::new(backend)?;
        terminal.resize(ratatui::layout::Rect {
            x: 0,
            y: 0,
            width: w,
            height: h,
        })?;
        terminal.draw(|f| {
            draw_ui(f, app, theme);
        })?;
    }

    // Write the collected bytes to the SSH channel
    app.write_bytes(&buf);
    Ok(())
}

fn draw_ui(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme) {
    let size = f.size();

    // Main background
    let bg_style = Style::default().bg(theme.bg).fg(theme.fg);
    f.render_widget(
        Block::default().style(bg_style),
        size,
    );

    // Top bar + content + bottom bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    draw_top_bar(f, app, theme, chunks[0]);

    // Three-pane layout
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18),
            Constraint::Min(0),
            Constraint::Length(22),
        ])
        .split(chunks[1]);

    draw_sidebar(f, app, theme, main_chunks[0]);
    draw_main(f, app, theme, main_chunks[1]);
    draw_right_panel(f, app, theme, main_chunks[2]);

    draw_bottom_bar(f, app, theme, chunks[2]);

    // Modal overlays
    if app.app_state.modal != Modal::None {
        draw_modal(f, app, theme, size);
    }
}

fn draw_top_bar(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    let active_count = app.state.active_users.try_lock()
        .map(|g| g.len())
        .unwrap_or(1);

    let nav = " home.sh  [d]ash [c]hat [a]rcade [p]rofile [?]help";
    let online_str = format!("● {} online ", active_count);

    let pad = area.width.saturating_sub(nav.len() as u16 + online_str.len() as u16);

    let line = Line::from(vec![
        Span::styled(nav, Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        Span::raw(" ".repeat(pad as usize)),
        Span::styled(online_str, Style::default().fg(theme.success)),
    ]);
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme.border).fg(theme.fg)),
        area,
    );
}

fn draw_bottom_bar(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    let screen_hints = match app.app_state.screen {
        Screen::Chat => {
            if app.app_state.input_mode {
                " [ESC]cancel [Enter]send"
            } else {
                " [i]nput [h/l]rooms [j/k]scroll [q]quit"
            }
        }
        Screen::Dashboard => " [j/k]scroll [q]quit",
        Screen::Arcade => " [s]nake [w]ordle [q]quit",
        Screen::Profile => " [t]heme [b]io [q]quit",
        Screen::Help => " [q]close",
    };
    let right = "thanks for hanging out ";

    let pad = area.width.saturating_sub(screen_hints.len() as u16 + right.len() as u16);

    let line = Line::from(vec![
        Span::styled(screen_hints, Style::default().fg(theme.dim)),
        Span::raw(" ".repeat(pad as usize)),
        Span::styled(right, Style::default().fg(theme.accent).add_modifier(Modifier::ITALIC)),
    ]);
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme.border).fg(theme.fg)),
        area,
    );
}

fn draw_sidebar(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut items: Vec<ListItem> = Vec::new();

    items.push(ListItem::new(Line::from(Span::styled(
        " ── Rooms ──",
        Style::default().fg(theme.dim).add_modifier(Modifier::BOLD),
    ))));

    for (i, room) in app.app_state.rooms.iter().enumerate() {
        let is_selected = i == app.app_state.current_room && app.app_state.screen == Screen::Chat;
        let prefix = if is_selected { " > " } else { "   " };
        let style = if is_selected {
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg)
        };
        items.push(ListItem::new(Line::from(Span::styled(
            format!("{}#{}", prefix, room.name),
            style,
        ))));
    }

    items.push(ListItem::new(Line::from(Span::raw(""))));
    items.push(ListItem::new(Line::from(Span::styled(
        " ── Online ──",
        Style::default().fg(theme.dim).add_modifier(Modifier::BOLD),
    ))));

    items.push(ListItem::new(Line::from(Span::styled(
        format!(" ● {}", app.app_state.user.username),
        Style::default().fg(theme.success),
    ))));

    let list = List::new(items).style(Style::default().bg(theme.bg).fg(theme.fg));
    f.render_widget(list, inner);
}

fn draw_main(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    match app.app_state.screen {
        Screen::Dashboard => crate::app::screens::dashboard::draw(f, app, theme, area),
        Screen::Chat => crate::app::screens::chat::draw(f, app, theme, area),
        Screen::Profile => crate::app::screens::profile::draw(f, app, theme, area),
        Screen::Arcade => crate::app::screens::arcade::draw(f, app, theme, area),
        Screen::Help => draw_help(f, app, theme, area),
    }
}

fn draw_help(f: &mut Frame, _app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let help_text = vec![
        Line::from(Span::styled("Navigation", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  d - Dashboard")),
        Line::from(Span::raw("  c - Chat")),
        Line::from(Span::raw("  a - Arcade")),
        Line::from(Span::raw("  p - Profile")),
        Line::from(Span::raw("  q - Quit")),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Cat Care", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  f - Feed cat")),
        Line::from(Span::raw("  w - Water cat")),
        Line::from(Span::raw("  g - Groom cat")),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Chat", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  i - Input mode")),
        Line::from(Span::raw("  h/l - Switch rooms")),
        Line::from(Span::raw("  j/k - Scroll messages")),
        Line::from(Span::raw("  ESC - Exit input")),
    ];

    let para = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false });

    f.render_widget(para, area);
}

fn draw_right_panel(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // Clock
            Constraint::Length(6),  // Now playing
            Constraint::Min(8),     // Cat
            Constraint::Length(5),  // Visualizer
        ])
        .split(inner);

    crate::app::widgets::clock::draw(f, app, theme, chunks[0]);
    crate::app::widgets::now_playing::draw(f, app, theme, chunks[1]);
    crate::app::widgets::cat::draw(f, app, theme, chunks[2]);
    crate::app::widgets::visualizer::draw(f, app, theme, chunks[3]);
}

fn draw_modal(f: &mut Frame, app: &mut App, theme: &crate::app::theme::Theme, area: Rect) {
    match app.app_state.modal {
        Modal::Help => {
            let modal_area = centered_rect(60, 70, area);
            draw_help(f, app, theme, modal_area);
        }
        Modal::ThemePicker => {
            let modal_area = centered_rect(50, 80, area);
            crate::app::screens::profile::draw_theme_picker(f, app, theme, modal_area);
        }
        _ => {}
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
