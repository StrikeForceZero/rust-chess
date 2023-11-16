use bevy::prelude::*;
use itertools::Itertools;
use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::chess_move::chess_move_search::unchecked_chess_move_search_from_pos;

use super::selection;
use super::super::state::CurrentGame;

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub enum SquareColor {
    Dark,
    Light,
    Highlight,
    HighlightDanger,
    LastMoveFrom,
    LastMoveTo,
    LegalMove,
    LegalAttack,
    NoMove,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct Calculated;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                clear_colors
                    .after(selection::Calculated)
                    .in_set(Calculated)
            )
            .add_systems(
                Update,
                assign_colors
                    .after(selection::Calculated)
                    .in_set(Calculated)
            );
    }
}

fn get_check_pos(
    current: &Res<CurrentGame>,
) -> Option<BoardPosition> {
    use crate::color::Color::*;
    let check = current.state().game_status.is_check_or_mate();
    if check {
        let color_in_check = current.state().active_color;
        let bit_boards = current.board().as_bit_boards_const();
        let bit_boards = match color_in_check {
            White => bit_boards.white,
            Black => bit_boards.black,
        };
        if let Some(index) = bit_boards.king.as_bitboard().bitmap().first_index() {
            match BoardPosition::from_pos_index(index) {
                Ok(pos) => Some(pos),
                Err(err) => {
                    warn!("failed to get pos for index: {index} - {err:?}");
                    None
                }
            }
        } else {
            warn!("in check but can't find {color_in_check} king pos");
            None
        }
    } else {
        None
    }
}

fn clear_colors(
    current: Res<CurrentGame>,
    mut q_square: Query<(&BoardRank, &BoardFile, &mut SquareColor)>,
) {
    if !current.is_changed() || current.selected().is_some() {
        return;
    }
    let last_move = current.state().history.move_history.last();
    let maybe_check_pos = get_check_pos(&current);
    debug!(?maybe_check_pos, "Checking board");
    for (&rank, &file, mut color) in q_square.iter_mut() {
        let pos = BoardPosition::from(file, rank);
        let new_color = if maybe_check_pos.map_or(false, |check| check == pos) {
            Some(SquareColor::HighlightDanger)
        } else if last_move.map_or(false, |last_move| last_move.from == pos) {
            Some(SquareColor::LastMoveFrom)
        } else if last_move.map_or(false, |last_move| last_move.to == pos) {
            Some(SquareColor::LastMoveTo)
        } else {
            None
        };
        if let Some(new_color) = new_color {
            if *color == new_color {
                continue;
            }
            *color = new_color;
            continue;
        }
        match *color {
            SquareColor::Light |
            SquareColor::Dark =>
                continue,
            _ => {}
        }
        *color = SquareColor::from_file_rank(file, rank);
    }
}

fn assign_colors(
    current: Res<CurrentGame>,
    mut q_square: Query<(&BoardRank, &BoardFile, &mut SquareColor)>,
) {
    if !current.is_changed() {
        return;
    }
    let Some((selected_rank, selected_file)) = current.selected() else {
        return;
    };
    let chess_moves = unchecked_chess_move_search_from_pos(current.state(), BoardPosition::from(selected_file, selected_rank), None);
    // TODO: implement methods used in move_handler to get legal moves
    let provisional_moves = chess_moves.iter().filter(|m| m.captured_piece.is_none()).collect_vec();
    // TODO: implement methods used in move_handler to get legal moves
    let provisional_attacks = chess_moves.iter().filter(|m| m.captured_piece.is_some()).collect_vec();

    let maybe_check_pos = get_check_pos(&current);
    debug!(?maybe_check_pos, "Checking board");
    for (rank, file, mut color) in q_square.iter_mut() {
        let (rank, file) = (*rank, *file);
        let pos = BoardPosition::from(file, rank);
        let new_color = if rank == selected_rank && file == selected_file {
            if provisional_moves.len() > 0 || provisional_attacks.len() > 0 {
                SquareColor::Highlight
            } else {
                SquareColor::NoMove
            }
        } else if provisional_moves.iter().find(|m| m.to == pos).is_some() {
            SquareColor::LegalMove
        } else if provisional_attacks.iter().find(|m| m.to == pos).is_some() {
            SquareColor::LegalAttack
        } else if maybe_check_pos.map_or(false, |check| check == pos) {
            SquareColor::HighlightDanger
        } else {
            SquareColor::from_file_rank(file, rank)
        };
        if *color == new_color {
            continue;
        }
        *color = new_color
    }
}

impl SquareColor {
    pub fn as_color(&self) -> Color {
        match self {
            SquareColor::Dark => Color::hex("769656").unwrap(),
            SquareColor::Light => Color::hex("eeeed2").unwrap(),
            SquareColor::Highlight => Color::hex("baca44").unwrap(),
            SquareColor::HighlightDanger => Color::hex("800020").unwrap(),
            SquareColor::LastMoveFrom => Color::hex("44ca51").unwrap(),
            SquareColor::LastMoveTo => Color::hex("44ca51").unwrap(),
            SquareColor::LegalMove => Color::hex("52dfff").unwrap(),
            SquareColor::LegalAttack => Color::hex("ffbe52").unwrap(),
            SquareColor::NoMove => Color::hex("313131").unwrap(),
        }
    }

    pub fn from_file_rank(file: BoardFile, rank: BoardRank) -> SquareColor {
        if file.as_usize() % 2 ^ rank.as_usize() % 2 == 0 {
            Self::Dark
        } else {
            Self::Light
        }
    }

    pub fn from_row_col(row_ix: usize, col_ix: usize) -> SquareColor {
        if row_ix % 2 ^ col_ix % 2 == 0 {
            Self::Dark
        } else {
            Self::Light
        }
    }

    pub fn as_inverse(&self) -> SquareColor {
        match self {
            SquareColor::Dark => SquareColor::Light,
            SquareColor::Light => SquareColor::Dark,
            SquareColor::Highlight => SquareColor::Light,
            SquareColor::HighlightDanger => SquareColor::Light,
            SquareColor::LastMoveFrom => SquareColor::Light,
            SquareColor::LastMoveTo => SquareColor::Light,
            SquareColor::LegalMove => SquareColor::Light,
            SquareColor::LegalAttack => SquareColor::Light,
            SquareColor::NoMove => SquareColor::Light,
        }
    }
}
