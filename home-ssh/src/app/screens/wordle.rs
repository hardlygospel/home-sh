use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::theme::Theme;

const WORD_LIST: &[&str] = &[
    "CRANE", "SLATE", "AUDIO", "AROSE", "STALE", "TRAIN", "IRATE",
    "ARISE", "SNARE", "TRACE", "CRATE", "GROAN", "PLAIN", "BRAIN",
    "GRAIN", "TRAIL", "PLAID", "SAINT", "BLAZE", "GLARE", "FLARE",
    "SHARE", "SPARE", "STARE", "SCARE", "SNARE", "GRADE", "TRADE",
    "SHADE", "BLADE", "PLACE", "GRACE", "BRACE", "PRICE", "SLICE",
    "SPICE", "TWICE", "VOICE", "POINT", "JOINT", "FRONT", "PLANT",
    "GRANT", "SLANT", "CHANT", "GIANT", "CHAIR", "STAIR", "FLAIR",
    "CLAIM", "FLAME", "FRAME", "SHAME", "SWAMP", "STOMP", "STOIC",
    "CIVIC", "MAGIC", "BASIC", "PANIC", "MANIC", "ETHIC", "LYRIC",
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LetterState {
    Unknown,
    Correct,   // Green
    Present,   // Yellow
    Absent,    // Gray
}

#[derive(Debug, Clone)]
pub struct WordleGame {
    pub target: String,
    pub guesses: Vec<String>,
    pub current: String,
    pub states: Vec<Vec<LetterState>>,
    pub won: bool,
    pub lost: bool,
    pub message: Option<String>,
}

impl WordleGame {
    pub fn new() -> Self {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let target = WORD_LIST.choose(&mut rng).unwrap_or(&"CRANE").to_string();
        WordleGame {
            target,
            guesses: Vec::new(),
            current: String::new(),
            states: Vec::new(),
            won: false,
            lost: false,
            message: None,
        }
    }

    pub fn type_char(&mut self, ch: char) {
        if !self.won && !self.lost && self.current.len() < 5 {
            self.current.push(ch);
        }
    }

    pub fn backspace(&mut self) {
        self.current.pop();
    }

    pub fn submit_guess(&mut self) {
        if self.won || self.lost {
            return;
        }
        if self.current.len() != 5 {
            self.message = Some("Word must be 5 letters".to_string());
            return;
        }

        let guess = self.current.clone();
        let state = self.evaluate(&guess);
        let correct = state.iter().all(|s| *s == LetterState::Correct);

        self.guesses.push(guess.clone());
        self.states.push(state);
        self.current.clear();

        if correct {
            self.won = true;
            self.message = Some(format!("🎉 You got it in {}!", self.guesses.len()));
        } else if self.guesses.len() >= 6 {
            self.lost = true;
            self.message = Some(format!("The word was {}", self.target));
        } else {
            self.message = None;
        }
    }

    fn evaluate(&self, guess: &str) -> Vec<LetterState> {
        let target: Vec<char> = self.target.chars().collect();
        let guess_chars: Vec<char> = guess.chars().collect();
        let mut result = vec![LetterState::Absent; 5];
        let mut target_used = vec![false; 5];

        // First pass: exact matches
        for i in 0..5 {
            if guess_chars[i] == target[i] {
                result[i] = LetterState::Correct;
                target_used[i] = true;
            }
        }

        // Second pass: present but wrong position
        for i in 0..5 {
            if result[i] == LetterState::Correct {
                continue;
            }
            for j in 0..5 {
                if !target_used[j] && guess_chars[i] == target[j] {
                    result[i] = LetterState::Present;
                    target_used[j] = true;
                    break;
                }
            }
        }

        result
    }
}

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(" Wordle ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let wordle = match &app.app_state.wordle {
        Some(w) => w,
        None => return,
    };

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from(Span::raw("")));

    // Display 6 rows
    for row in 0..6 {
        let mut spans: Vec<Span> = vec![Span::raw("  ")];

        if row < wordle.guesses.len() {
            let guess: Vec<char> = wordle.guesses[row].chars().collect();
            let state = &wordle.states[row];
            for i in 0..5 {
                let (fg, bg) = match state[i] {
                    LetterState::Correct => (theme.bg, theme.success),
                    LetterState::Present => (theme.bg, theme.warning),
                    LetterState::Absent => (theme.fg, theme.dim),
                    LetterState::Unknown => (theme.fg, theme.bg),
                };
                spans.push(Span::styled(
                    format!(" {} ", guess[i]),
                    Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD),
                ));
                spans.push(Span::raw(" "));
            }
        } else if row == wordle.guesses.len() && !wordle.won && !wordle.lost {
            // Current input row
            let current: Vec<char> = wordle.current.chars().collect();
            for i in 0..5 {
                let ch = current.get(i).copied().unwrap_or(' ');
                let style = if i < current.len() {
                    Style::default().fg(theme.fg).bg(theme.border).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.dim).bg(theme.bg)
                };
                spans.push(Span::styled(format!(" {} ", ch), style));
                spans.push(Span::raw(" "));
            }
        } else {
            // Empty rows
            for _ in 0..5 {
                spans.push(Span::styled(
                    " _ ",
                    Style::default().fg(theme.dim),
                ));
                spans.push(Span::raw(" "));
            }
        }

        lines.push(Line::from(spans));
    }

    lines.push(Line::from(Span::raw("")));

    if let Some(msg) = &wordle.message {
        lines.push(Line::from(Span::styled(
            format!("  {}", msg),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        )));
    }

    if wordle.won || wordle.lost {
        lines.push(Line::from(Span::styled(
            "  [q] back to arcade",
            Style::default().fg(theme.dim),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "  Type letters, [Enter] to guess, [Backspace] to delete",
            Style::default().fg(theme.dim),
        )));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
