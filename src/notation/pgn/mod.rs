mod simple_parser;
mod pgn_move_detail;
mod pgn_turn_data;
mod pgn_roster_raw;
mod tag_pairs;
mod pgn_roster;
mod pgn_turn_builder;
mod pgn_move_detail_builder;
mod pgn_roster_raw_partial;
mod pgn_turn_data_raw;
mod pgn_turn_data_raw_partial;
mod pgn_move;
mod pgn_move_builder;
mod pgn_parsing_error;
mod util;
mod parser;
mod lexer;
mod pgn_data;
mod pgn_data_partial;

#[repr(transparent)]
#[derive(Clone)]
pub struct Pgn(String);
