use crate::app::App;
use crate::app::app_state::ArcadeGame;

pub async fn do_tick(app: &mut App) {
    // Status message countdown
    if app.app_state.status_timer > 0 {
        app.app_state.status_timer -= 1;
        if app.app_state.status_timer == 0 {
            app.app_state.status_msg = None;
        }
    }

    // Cat action message countdown
    if app.app_state.cat_action_timer > 0 {
        app.app_state.cat_action_timer -= 1;
        if app.app_state.cat_action_timer == 0 {
            app.app_state.cat_action_msg = None;
        }
    }

    // Update now_playing from shared state
    // (it's updated by background task, we just use it)

    // Visualizer animation
    tick_visualizer(app);

    // Snake tick
    if app.app_state.arcade_game == ArcadeGame::Snake {
        if let Some(snake) = &mut app.app_state.snake {
            snake.tick();
        }
    }
}

fn tick_visualizer(app: &mut App) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    app.app_state.viz_tick = app.app_state.viz_tick.wrapping_add(1);

    for (i, bar) in app.app_state.viz_bars.iter_mut().enumerate() {
        // Decay
        *bar = (*bar - 0.05).max(0.0);
        // Random spike
        if rng.gen_bool(0.15) {
            let target = rng.gen_range(0.2..1.0);
            if target > *bar {
                *bar = target;
            }
        }
    }
}
