use rand::Rng;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::collections::VecDeque;

use crate::app::App;
use crate::app::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

pub struct SnakeGame {
    pub snake: VecDeque<Point>,
    pub food: Point,
    pub direction: Direction,
    pub next_direction: Direction,
    pub score: u32,
    pub alive: bool,
    pub width: i16,
    pub height: i16,
    pub tick_count: u32,
    pub speed: u32,
}

impl SnakeGame {
    pub fn new() -> Self {
        let width = 40i16;
        let height = 20i16;
        let mut snake = VecDeque::new();
        snake.push_front(Point { x: width / 2, y: height / 2 });
        snake.push_back(Point { x: width / 2 - 1, y: height / 2 });

        let food = Point { x: width / 4, y: height / 4 };

        SnakeGame {
            snake,
            food,
            direction: Direction::Right,
            next_direction: Direction::Right,
            score: 0,
            alive: true,
            width,
            height,
            tick_count: 0,
            speed: 5,
        }
    }

    pub fn set_direction(&mut self, dir: Direction) {
        // Prevent 180-degree turns
        let invalid = match dir {
            Direction::Up => self.direction == Direction::Down,
            Direction::Down => self.direction == Direction::Up,
            Direction::Left => self.direction == Direction::Right,
            Direction::Right => self.direction == Direction::Left,
        };
        if !invalid {
            self.next_direction = dir;
        }
    }

    pub fn tick(&mut self) {
        if !self.alive {
            return;
        }

        self.tick_count += 1;
        if self.tick_count % self.speed != 0 {
            return;
        }

        self.direction = self.next_direction;

        let head = *self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point { x: head.x, y: head.y - 1 },
            Direction::Down => Point { x: head.x, y: head.y + 1 },
            Direction::Left => Point { x: head.x - 1, y: head.y },
            Direction::Right => Point { x: head.x + 1, y: head.y },
        };

        // Wall collision
        if new_head.x < 0 || new_head.x >= self.width || new_head.y < 0 || new_head.y >= self.height {
            self.alive = false;
            return;
        }

        // Self collision
        if self.snake.contains(&new_head) {
            self.alive = false;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 10;
            self.speed = (5 - (self.score / 50).min(4)) as u32;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let p = Point {
                x: rng.gen_range(0..self.width),
                y: rng.gen_range(0..self.height),
            };
            if !self.snake.contains(&p) {
                self.food = p;
                break;
            }
        }
    }
}

pub fn draw(f: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .title(format!(" Snake — Score: {} ", app.app_state.snake.as_ref().map(|s| s.score).unwrap_or(0)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let snake = match &app.app_state.snake {
        Some(s) => s,
        None => return,
    };

    if !snake.alive {
        let lines = vec![
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                "  GAME OVER",
                Style::default().fg(theme.error).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("  Score: {}", snake.score),
                Style::default().fg(theme.primary),
            )),
            Line::from(Span::styled(
                "  [q] back to menu",
                Style::default().fg(theme.dim),
            )),
        ];
        let para = Paragraph::new(lines);
        f.render_widget(para, inner);
        return;
    }

    // Render grid
    let grid_w = snake.width as usize;
    let grid_h = snake.height as usize;

    let mut lines: Vec<Line> = Vec::with_capacity(grid_h);

    for y in 0..grid_h {
        let mut spans: Vec<Span> = Vec::with_capacity(grid_w + 2);
        spans.push(Span::styled("│", Style::default().fg(theme.border)));
        for x in 0..grid_w {
            let p = Point { x: x as i16, y: y as i16 };
            if snake.snake.front() == Some(&p) {
                // Head
                spans.push(Span::styled("●", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));
            } else if snake.snake.contains(&p) {
                // Body
                spans.push(Span::styled("○", Style::default().fg(theme.secondary)));
            } else if snake.food == p {
                // Food
                spans.push(Span::styled("*", Style::default().fg(theme.warning)));
            } else {
                spans.push(Span::raw(" "));
            }
        }
        spans.push(Span::styled("│", Style::default().fg(theme.border)));
        lines.push(Line::from(spans));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(theme.bg));
    f.render_widget(para, inner);
}
