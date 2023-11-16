use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;
use crate::gui::plugins::*;
use crate::gui::systems::placed::PlacedPiece;
use crate::piece::piece::Piece;

mod plugins;
mod systems;

pub fn init() {
    use bevy::prelude::*;

    let mut app = App::new();
    app
        .add_plugins(view::Plugin)
        .add_plugins(board::Plugin)
        .add_plugins(clicks::Plugin)
        .add_plugins(state::Plugin)
        .add_plugins(pieces::Plugin)
        .add_plugins(bot::Plugin)
            .insert_resource(bot::BotConfig::black(2))

    ;
    app
        .register_type::<PlacedPiece>()
        .register_type::<Piece>()
        .register_type::<BoardFile>()
        .register_type::<BoardRank>()
    ;
    app.run();
}
