pub mod app_state;
pub mod input;
pub mod render;
pub mod theme;
pub mod tick;
pub mod screens;
pub mod widgets;

use std::sync::Arc;

use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use home_core::models::user::User;

use crate::ssh::SshWriter;
use crate::state::SharedState;
use app_state::AppState;

pub struct App {
    pub state: Arc<SharedState>,
    pub app_state: AppState,
    pub writer: SshWriter,
    pub input_rx: mpsc::Receiver<Vec<u8>>,
    pub resize_rx: mpsc::Receiver<(u16, u16)>,
    pub width: u16,
    pub height: u16,
}

impl App {
    pub fn new(
        state: Arc<SharedState>,
        user: User,
        writer: SshWriter,
        input_rx: mpsc::Receiver<Vec<u8>>,
        resize_rx: mpsc::Receiver<(u16, u16)>,
    ) -> Self {
        let app_state = AppState::new(user);
        App {
            state,
            app_state,
            writer,
            input_rx,
            resize_rx,
            width: 120,
            height: 40,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Send initial terminal setup
        self.write_bytes(b"\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H");

        // Load initial data
        self.load_rooms().await;
        self.load_cat().await;
        self.load_messages().await;

        let mut tick_interval = tokio::time::interval(tokio::time::Duration::from_millis(66));
        let mut chat_rx = self.state.chat_broadcast.subscribe();

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    self.tick().await;
                    self.render().await?;
                }
                Some(bytes) = self.input_rx.recv() => {
                    let should_quit = self.handle_input(&bytes).await;
                    if should_quit {
                        break;
                    }
                    self.render().await?;
                }
                Some((w, h)) = self.resize_rx.recv() => {
                    self.width = w;
                    self.height = h;
                    self.render().await?;
                }
                Ok(msg) = chat_rx.recv() => {
                    if msg.room_id == self.app_state.current_room_id() {
                        self.app_state.add_message(msg);
                        self.render().await?;
                    }
                }
            }
        }

        // Restore terminal on exit
        self.write_bytes(b"\x1b[?1049l\x1b[?25h\x1b[2J\x1b[H");

        Ok(())
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        use std::io::Write;
        let _ = self.writer.write_all(data);
        let _ = self.writer.flush();
    }

    pub async fn load_rooms(&mut self) {
        if let Ok(rooms) = self.state.chat_svc.get_rooms().await {
            self.app_state.rooms = rooms;
            if !self.app_state.rooms.is_empty() {
                self.app_state.current_room = 0;
            }
        }
    }

    pub async fn load_cat(&mut self) {
        let user_id = self.app_state.user.id;
        if let Ok(cat) = self.state.cat_svc.get_or_create_cat(user_id).await {
            self.app_state.cat = Some(cat);
        }
    }

    pub async fn load_messages(&mut self) {
        if let Some(room) = self.app_state.rooms.get(self.app_state.current_room) {
            let room_id = room.id;
            if let Ok(msgs) = self.state.chat_svc.get_messages(room_id, 100).await {
                self.app_state.messages = msgs;
                self.app_state.msg_scroll = 0;
            }
        }
    }

    async fn tick(&mut self) {
        tick::do_tick(self).await;
    }

    async fn handle_input(&mut self, bytes: &[u8]) -> bool {
        input::handle_input(self, bytes).await
    }

    async fn render(&mut self) -> Result<()> {
        render::render(self).await
    }
}

/// A raw pointer wrapper to allow the SshWriter to be used in the terminal backend.
/// Safe because the terminal is only used within the render() call synchronously.
pub struct SshWriterRef {
    writer: *mut SshWriter,
}

impl std::io::Write for SshWriterRef {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe { (*self.writer).write(buf) }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        unsafe { (*self.writer).flush() }
    }
}
