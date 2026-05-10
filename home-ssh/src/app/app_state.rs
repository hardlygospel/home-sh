use std::collections::HashMap;
use uuid::Uuid;

use home_core::models::cat::Cat;
use home_core::models::chat::{ChatMessage, ChatRoom};
use home_core::models::user::User;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Dashboard,
    Chat,
    Profile,
    Arcade,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArcadeGame {
    Menu,
    Snake,
    Wordle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Modal {
    None,
    Help,
    ThemePicker,
    BioEdit,
}

pub struct AppState {
    pub user: User,
    pub screen: Screen,
    pub modal: Modal,

    // Chat
    pub rooms: Vec<ChatRoom>,
    pub current_room: usize,
    pub messages: Vec<ChatMessage>,
    pub msg_scroll: usize,
    pub input_buf: String,
    pub input_mode: bool,

    // Cat
    pub cat: Option<Cat>,
    pub cat_action_msg: Option<String>,
    pub cat_action_timer: u32,

    // Theme picker
    pub theme_picker_idx: usize,

    // Profile
    pub bio_buf: String,
    pub bio_editing: bool,

    // Arcade
    pub arcade_game: ArcadeGame,
    pub snake: Option<crate::app::screens::snake::SnakeGame>,
    pub wordle: Option<crate::app::screens::wordle::WordleGame>,

    // Dashboard
    pub hn_scroll: usize,

    // Status messages
    pub status_msg: Option<String>,
    pub status_timer: u32,

    // Visualizer state
    pub viz_bars: [f32; 8],
    pub viz_tick: u32,
}

impl AppState {
    pub fn new(user: User) -> Self {
        let theme_id = user.theme_id.clone();
        AppState {
            user,
            screen: Screen::Dashboard,
            modal: Modal::None,
            rooms: Vec::new(),
            current_room: 0,
            messages: Vec::new(),
            msg_scroll: 0,
            input_buf: String::new(),
            input_mode: false,
            cat: None,
            cat_action_msg: None,
            cat_action_timer: 0,
            theme_picker_idx: crate::app::theme::theme_index_by_id(&theme_id),
            bio_buf: String::new(),
            bio_editing: false,
            arcade_game: ArcadeGame::Menu,
            snake: None,
            wordle: None,
            hn_scroll: 0,
            status_msg: None,
            status_timer: 0,
            viz_bars: [0.0; 8],
            viz_tick: 0,
        }
    }

    pub fn current_room_id(&self) -> Uuid {
        self.rooms
            .get(self.current_room)
            .map(|r| r.id)
            .unwrap_or_else(Uuid::new_v4)
    }

    pub fn current_room_name(&self) -> &str {
        self.rooms
            .get(self.current_room)
            .map(|r| r.name.as_str())
            .unwrap_or("general")
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        if self.messages.len() > 500 {
            self.messages.remove(0);
        }
    }

    #[allow(dead_code)]
    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_msg = Some(msg.into());
        self.status_timer = 60; // ~4 seconds at 66ms tick
    }
}
