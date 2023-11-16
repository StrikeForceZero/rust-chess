use bevy::prelude::*;
use bevy::render::RenderSet::{Queue, Render};
use bevy::tasks::{AsyncComputeTaskPool, block_on, Task, TaskPool};
use crate::chess_move::chess_move::ChessMove;
use crate::chess_move::chess_move_handler::default_chess_move_handler;
use crate::chess_move::invalid_chess_move_error::InvalidChessMoveError;
use crate::gui::plugins::state::CurrentGame;
use crate::state::evaluate_game_state::find_best_move;
use futures_lite::future;

#[derive(Resource, Clone, Debug, Default)]
pub struct BotConfig {
    pub color_to_play: Option<crate::color::Color>,
    pub depth: u8,
}

impl BotConfig {
    pub fn white(depth: u8) -> Self {
        Self {
            color_to_play: Some(crate::color::Color::White),
            depth,
        }
    }
    pub fn black(depth: u8) -> Self {
        Self {
            color_to_play: Some(crate::color::Color::Black),
            depth,
        }
    }
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BotConfig>()
            .add_systems(Update, bot_move_system)
            .add_systems(Update, handle_tasks)
        ;
    }
}

#[derive(Component)]
struct ComputeBestMove(Task<Result<ChessMove, &'static str>>);

fn bot_move_system(
    mut commands: Commands,
    mut current: ResMut<CurrentGame>,
    bot_config: Res<BotConfig>,
) {
    let Some(color_to_play) = bot_config.color_to_play else {
        return;
    };
    if !current.is_changed() || current.state().game_status.is_game_over() {
        return;
    }
    if current.state().active_color != color_to_play {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();
    let game_state_copy = current.state().clone();
    let bot_config_copy = bot_config.clone();
    let task = thread_pool.spawn(async move {
        debug!("bot task: trying to find best move, starting");
        let res = find_best_move(&game_state_copy, bot_config_copy.depth);
        debug!("bot task: trying to find best move, complete");
        res
    });
    commands.spawn(ComputeBestMove(task));
}

fn handle_tasks(
    mut commands: Commands,
    mut current: ResMut<CurrentGame>,
    mut transform_tasks: Query<(Entity, &mut ComputeBestMove)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(find_best_move_result) = block_on(future::poll_once(&mut task.0)) {
            let chess_move = match find_best_move_result {
                Ok(chess_move) => chess_move,
                Err(err) => {
                    warn!("failed to find_move: {err:?}");
                    return;
                }
            };
            debug!("best move {} -> {}", chess_move.from, chess_move.to);

            if let Err(error) = default_chess_move_handler(
                current.state_mut(),
                &chess_move,
                None,
            ) {
                warn!("best move was invalid: {error:?} {chess_move:?}");
            } else {
                current.clear_selected();
            }
            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<ComputeBestMove>();
        }
    }
}
