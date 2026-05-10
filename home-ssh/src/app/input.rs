use crate::app::app_state::{ArcadeGame, Modal, Screen};
use crate::app::App;

pub async fn handle_input(app: &mut App, bytes: &[u8]) -> bool {
    // Handle modal first
    if app.app_state.modal != Modal::None {
        return handle_modal_input(app, bytes).await;
    }

    // Screen-specific input
    match app.app_state.screen {
        Screen::Chat => {
            if app.app_state.input_mode {
                return handle_chat_input(app, bytes).await;
            }
        }
        Screen::Profile => {
            if app.app_state.bio_editing {
                return handle_bio_input(app, bytes).await;
            }
        }
        Screen::Arcade => {
            if app.app_state.arcade_game == ArcadeGame::Snake {
                return handle_snake_input(app, bytes).await;
            } else if app.app_state.arcade_game == ArcadeGame::Wordle {
                return handle_wordle_input(app, bytes).await;
            }
        }
        _ => {}
    }

    // Global navigation
    handle_global_input(app, bytes).await
}

async fn handle_global_input(app: &mut App, bytes: &[u8]) -> bool {
    match bytes {
        b"q" | b"\x03" => return true, // quit or Ctrl+C
        b"d" => {
            app.app_state.screen = Screen::Dashboard;
        }
        b"c" => {
            app.app_state.screen = Screen::Chat;
            if app.app_state.rooms.is_empty() {
                if let Ok(rooms) = app.state.chat_svc.get_rooms().await {
                    app.app_state.rooms = rooms;
                }
            }
            app.load_messages().await;
        }
        b"a" => {
            app.app_state.screen = Screen::Arcade;
            app.app_state.arcade_game = ArcadeGame::Menu;
        }
        b"p" => {
            app.app_state.screen = Screen::Profile;
            app.app_state.bio_buf = app.app_state.user.bio.clone().unwrap_or_default();
        }
        b"?" => {
            app.app_state.modal = Modal::Help;
        }
        // Chat: switch rooms
        b"h" => {
            if app.app_state.screen == Screen::Chat && app.app_state.current_room > 0 {
                app.app_state.current_room -= 1;
                app.load_messages().await;
            } else if app.app_state.screen == Screen::Dashboard {
                if app.app_state.hn_scroll > 0 {
                    app.app_state.hn_scroll -= 1;
                }
            }
        }
        b"l" => {
            if app.app_state.screen == Screen::Chat
                && app.app_state.current_room + 1 < app.app_state.rooms.len()
            {
                app.app_state.current_room += 1;
                app.load_messages().await;
            }
        }
        b"j" => {
            match app.app_state.screen {
                Screen::Chat => {
                    if app.app_state.msg_scroll + 1 < app.app_state.messages.len() {
                        app.app_state.msg_scroll += 1;
                    }
                }
                Screen::Dashboard => {
                    app.app_state.hn_scroll += 1;
                }
                _ => {}
            }
        }
        b"k" => {
            match app.app_state.screen {
                Screen::Chat => {
                    if app.app_state.msg_scroll > 0 {
                        app.app_state.msg_scroll -= 1;
                    }
                }
                Screen::Dashboard => {
                    if app.app_state.hn_scroll > 0 {
                        app.app_state.hn_scroll -= 1;
                    }
                }
                _ => {}
            }
        }
        b"i" => {
            if app.app_state.screen == Screen::Chat {
                app.app_state.input_mode = true;
            }
        }
        // Cat care commands
        b"f" => {
            let uid = app.app_state.user.id;
            if let Ok(cat) = app.state.cat_svc.feed(uid).await {
                app.app_state.cat = Some(cat);
                app.app_state.cat_action_msg = Some("Fed! nom nom nom".to_string());
                app.app_state.cat_action_timer = 45;
            }
        }
        b"w" => {
            let uid = app.app_state.user.id;
            if let Ok(cat) = app.state.cat_svc.water(uid).await {
                app.app_state.cat = Some(cat);
                app.app_state.cat_action_msg = Some("Watered! slurp slurp".to_string());
                app.app_state.cat_action_timer = 45;
            }
        }
        b"g" => {
            let uid = app.app_state.user.id;
            if let Ok(cat) = app.state.cat_svc.groom(uid).await {
                app.app_state.cat = Some(cat);
                app.app_state.cat_action_msg = Some("Groomed! purrrr".to_string());
                app.app_state.cat_action_timer = 45;
            }
        }
        _ => {}
    }

    false
}

async fn handle_modal_input(app: &mut App, bytes: &[u8]) -> bool {
    match bytes {
        b"q" | b"\x1b" | b"\r" | b"\n" => {
            if app.app_state.modal == Modal::ThemePicker {
                // Save theme
                let idx = app.app_state.theme_picker_idx;
                let theme_id = crate::app::theme::ALL_THEMES[idx].id.to_string();
                let uid = app.app_state.user.id;
                app.app_state.user.theme_id = theme_id.clone();
                let _ = app.state.user_svc.update_theme(uid, &theme_id).await;
            }
            app.app_state.modal = Modal::None;
        }
        b"j" | b"\x1b[B" => {
            if app.app_state.modal == Modal::ThemePicker {
                let len = crate::app::theme::ALL_THEMES.len();
                app.app_state.theme_picker_idx = (app.app_state.theme_picker_idx + 1) % len;
            }
        }
        b"k" | b"\x1b[A" => {
            if app.app_state.modal == Modal::ThemePicker {
                let len = crate::app::theme::ALL_THEMES.len();
                if app.app_state.theme_picker_idx == 0 {
                    app.app_state.theme_picker_idx = len - 1;
                } else {
                    app.app_state.theme_picker_idx -= 1;
                }
            }
        }
        _ => {}
    }
    false
}

async fn handle_chat_input(app: &mut App, bytes: &[u8]) -> bool {
    match bytes {
        b"\x1b" => {
            app.app_state.input_mode = false;
        }
        b"\r" | b"\n" => {
            let body = app.app_state.input_buf.trim().to_string();
            if !body.is_empty() {
                let room_id = app.app_state.current_room_id();
                let user_id = app.app_state.user.id;
                if let Ok(msg) = app.state.chat_svc.send_message(room_id, user_id, &body, None).await {
                    app.app_state.add_message(msg.clone());
                    let _ = app.state.chat_broadcast.send(msg);
                }
                app.app_state.input_buf.clear();
            }
        }
        b"\x7f" | b"\x08" => {
            app.app_state.input_buf.pop();
        }
        _ => {
            if let Ok(s) = std::str::from_utf8(bytes) {
                for ch in s.chars() {
                    if ch.is_control() {
                        continue;
                    }
                    app.app_state.input_buf.push(ch);
                }
            }
        }
    }
    false
}

async fn handle_bio_input(app: &mut App, bytes: &[u8]) -> bool {
    match bytes {
        b"\x1b" => {
            app.app_state.bio_editing = false;
        }
        b"\r" | b"\n" => {
            let bio = app.app_state.bio_buf.trim().to_string();
            let uid = app.app_state.user.id;
            if let Ok(()) = app.state.user_svc.update_bio(uid, &bio).await {
                app.app_state.user.bio = Some(bio);
            }
            app.app_state.bio_editing = false;
        }
        b"\x7f" | b"\x08" => {
            app.app_state.bio_buf.pop();
        }
        _ => {
            if let Ok(s) = std::str::from_utf8(bytes) {
                for ch in s.chars() {
                    if !ch.is_control() {
                        app.app_state.bio_buf.push(ch);
                    }
                }
            }
        }
    }
    false
}

async fn handle_snake_input(app: &mut App, bytes: &[u8]) -> bool {
    use crate::app::screens::snake::Direction;
    if let Some(snake) = &mut app.app_state.snake {
        match bytes {
            b"h" | b"\x1b[D" => snake.set_direction(Direction::Left),
            b"l" | b"\x1b[C" => snake.set_direction(Direction::Right),
            b"k" | b"\x1b[A" => snake.set_direction(Direction::Up),
            b"j" | b"\x1b[B" => snake.set_direction(Direction::Down),
            b"q" | b"\x1b" => {
                app.app_state.arcade_game = ArcadeGame::Menu;
                app.app_state.snake = None;
            }
            _ => {}
        }
    }
    false
}

async fn handle_wordle_input(app: &mut App, bytes: &[u8]) -> bool {
    if let Some(wordle) = &mut app.app_state.wordle {
        match bytes {
            b"\r" | b"\n" => {
                wordle.submit_guess();
            }
            b"\x7f" | b"\x08" => {
                wordle.backspace();
            }
            b"q" | b"\x1b" => {
                app.app_state.arcade_game = ArcadeGame::Menu;
                app.app_state.wordle = None;
            }
            _ => {
                if let Ok(s) = std::str::from_utf8(bytes) {
                    for ch in s.chars() {
                        if ch.is_ascii_alphabetic() {
                            wordle.type_char(ch.to_ascii_uppercase());
                        }
                    }
                }
            }
        }
    } else {
        match bytes {
            b"s" => {
                app.app_state.arcade_game = ArcadeGame::Snake;
                app.app_state.snake = Some(crate::app::screens::snake::SnakeGame::new());
            }
            b"w" => {
                app.app_state.arcade_game = ArcadeGame::Wordle;
                app.app_state.wordle = Some(crate::app::screens::wordle::WordleGame::new());
            }
            b"q" | b"\x1b" => {
                app.app_state.screen = Screen::Dashboard;
            }
            _ => {}
        }
    }
    false
}
