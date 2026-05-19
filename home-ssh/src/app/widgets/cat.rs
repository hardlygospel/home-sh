use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use home_core::models::cat::CatMood;
use crate::app::App;
use crate::app::theme::Theme;

// 7 ASCII cat stages: kitten → full grown cat
// Each stage has art lines
static CAT_STAGES: &[&[&str]] = &[
    // Stage 0: tiny kitten
    &[
        "  . .  ",
        " (o o) ",
        "  >^<  ",
    ],
    // Stage 1: small kitten
    &[
        "  /\\_  ",
        " (^o^) ",
        "  UUU  ",
    ],
    // Stage 2: young cat
    &[
        " /\\_/\\ ",
        "( ^.^ )",
        " (___) ",
    ],
    // Stage 3: adolescent cat
    &[
        " /\\_/\\  ",
        "( o.o ) ",
        " > ^ <  ",
        "  | |   ",
    ],
    // Stage 4: adult cat (sitting)
    &[
        "  /\\_/\\  ",
        " ( =.= ) ",
        " ( > < ) ",
        "  |   |  ",
        " /|   |\\ ",
    ],
    // Stage 5: large cat
    &[
        "   /\\_/\\   ",
        "  ( @.@ )  ",
        " / >   < \\ ",
        "  |  U  |  ",
        "  |_____|  ",
    ],
    // Stage 6: majestic full cat
    &[
        "   /\\_____/\\   ",
        "  /  o   o  \\  ",
        " ( ==  ^  == ) ",
        "  )         (  ",
        " (           ) ",
        "  \\  ~~~~~  /  ",
        "   \\_______/   ",
    ],
];

// Mood-specific expressions for the face
#[allow(dead_code)]
fn mood_face(mood: &CatMood) -> &'static str {
    match mood {
        CatMood::Happy => "^.^",
        CatMood::Sleeping => "-.-",
        CatMood::Hungry => "^o^",
        CatMood::Thirsty => "ToT",
        CatMood::Bored => "=_=",
        CatMood::Sad => "T_T",
    }
}

fn mood_color(mood: &CatMood, theme: &Theme) -> ratatui::style::Color {
    match mood {
        CatMood::Happy => theme.success,
        CatMood::Sleeping => theme.dim,
        CatMood::Hungry => theme.warning,
        CatMood::Thirsty => theme.accent,
        CatMood::Bored => theme.secondary,
        CatMood::Sad => theme.error,
    }
}

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Cat ")
        .borders(Borders::TOP)
        .border_style(Style::default().fg(theme.border));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cat = match &app.app_state.cat {
        Some(c) => c.clone(),
        None => {
            let para = Paragraph::new(vec![
                Line::from(Span::styled(" No cat!", Style::default().fg(theme.dim))),
            ]);
            f.render_widget(para, inner);
            return;
        }
    };

    let stage = cat.stage() as usize;
    let mood = cat.mood();
    let stage_art = CAT_STAGES[stage.min(CAT_STAGES.len() - 1)];
    let mood_color = mood_color(&mood, theme);

    let mut lines: Vec<Line> = Vec::new();

    // Render ASCII art
    for art_line in stage_art {
        lines.push(Line::from(Span::styled(
            art_line.to_string(),
            Style::default().fg(mood_color),
        )));
    }

    // Mood label
    lines.push(Line::from(vec![
        Span::styled(" mood: ", Style::default().fg(theme.dim)),
        Span::styled(
            mood.as_str(),
            Style::default().fg(mood_color).add_modifier(Modifier::BOLD),
        ),
    ]));

    // Growth progress
    let progress = cat.growth_points as f32 / 700.0;
    let bar_width = 10usize;
    let filled = (progress * bar_width as f32) as usize;
    let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    lines.push(Line::from(vec![
        Span::styled(" grow: ", Style::default().fg(theme.dim)),
        Span::styled(bar, Style::default().fg(theme.primary)),
        Span::styled(
            format!(" s{}", stage),
            Style::default().fg(theme.accent),
        ),
    ]));

    // Action message if present
    if let Some(msg) = &app.app_state.cat_action_msg {
        lines.push(Line::from(Span::styled(
            format!(" {}", msg),
            Style::default().fg(theme.success).add_modifier(Modifier::ITALIC),
        )));
    }

    lines.push(Line::from(Span::raw("")));
    lines.push(Line::from(Span::styled(
        " [f]eed [w]ater [g]room",
        Style::default().fg(theme.dim),
    )));

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
