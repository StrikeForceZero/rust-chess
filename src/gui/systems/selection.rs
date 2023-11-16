use bevy::prelude::*;
use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::chess_move::chess_move_handler::{default_chess_move_handler, try_handle_chess_move_and_apply};
use crate::chess_move::chess_move_search::find_move;

use crate::gui::systems::placed::PlacedPiece;
use super::super::clicks;
use super::super::state;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct Calculated;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                calculate_selection
                    .after(clicks::Calculated)
                    .in_set(Calculated)
            );
    }
}

fn calculate_selection(
    mut current: ResMut<state::CurrentGame>,
    q_clicked: Query<
        (&BoardRank, &BoardFile, &clicks::Clicked),
        (With<PlacedPiece>, Changed<clicks::Clicked>),
    >,
) {
    let mut iter = q_clicked.iter().peekable();
    if iter.peek().is_none() {
        return;
    }
    let Some((&rank, &file, _)) = iter.find(|(_, _, clicked)| clicked.is_clicked()) else {
        if current.selected().is_some() {
            info!("Clearing selection");
            current.clear_selected();
        }
        debug!("No clicks and no selection");
        return;
    };
    if iter.any(|(_, _, clicked)| clicked.is_clicked()) {
        error!("Multiple click-change");
        return;
    }
    let pos = BoardPosition::from(file, rank);

    let color_to_move = current.state().active_color;
    let Some((old_rank, old_file)) = current.selected() else {
        let Some(chess_piece) = current.board().get(pos) else {
            info!("Click without chess_piece");
            return;
        };
        if chess_piece.as_color() != color_to_move {
            info!("Click wrong color");
            return;
        }
        info!(?rank, ?file, "Selecting new");
        current.set_selected(rank, file);
        return;
    };

    if old_rank == rank && old_file == file {
        info!(?rank, ?file, "No selection change");
        return;
    }
    if current
        .board()
        .get(pos)
        .map_or(false, |piece| piece.as_color() == color_to_move)
    {
        info!(?rank, ?file, ?old_rank, ?old_file, "Selecting new piece");
        current.set_selected(rank, file);
        return;
    }

    let chess_move = find_move(
        current.state(),
        BoardPosition::from(old_file, old_rank),
        BoardPosition::from(file, rank),
        None,
        // TODO: handle promotions
        None,
    );

    let chess_move = match chess_move {
        Ok(chess_move) => chess_move,
        Err(err) => {
            warn!("failed to find_move: {err:?}");
            return;
        }
    };

    if let Err(error) = try_handle_chess_move_and_apply(
        current.state_mut(),
        &chess_move,
        None,
    ) {
        warn!("Cannot move that way {error:?}");
    } else {
        info!(?rank, ?file, ?old_rank, ?old_file, "Moved and clearing");
        current.clear_selected();
    }
}
