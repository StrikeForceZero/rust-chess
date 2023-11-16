use std::iter;

use bevy::prelude::*;
use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;

use crate::gui::systems::*;
use crate::gui::systems::placed::PlacedPiece;
use super::view;
use super::clicks;

pub struct Plugin;

#[derive(Bundle)]
struct Square {
    file: BoardFile,
    rank: BoardRank,
    piece: PlacedPiece,
    clicked: clicks::Clicked,
    extras: SpatialBundle,
}

#[derive(Bundle, Default)]
struct EmptyPieceContainerBundle {
    container: PieceContainer,
    extras: SpatialBundle,
}

#[derive(Component, Default, Copy, Clone)]
pub struct PieceContainer;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_plugins(background::Plugin)
            .add_plugins(coloring::Plugin)
            .add_plugins(selection::Plugin);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let rank_and_files = BoardRank::iter().flat_map(|rank| iter::repeat(rank).zip(BoardFile::iter()));
    for (rank, file) in rank_and_files {
        let y = (rank.as_usize() as f32) * view::SQUARE_SIZE;
        let x = (file.as_usize() as f32) * view::SQUARE_SIZE;
        let transform = Transform::from_xyz(x, y, 0.);
        let mut square = commands.spawn(Square {
            rank,
            file,
            clicked: default(),
            piece: PlacedPiece::Empty,
            extras: SpatialBundle {
                transform,
                ..default()
            },
        });

        let font: Handle<Font> = asset_server.load("fonts/CourierPrime-Regular.ttf");
        square.with_children(move |mut parent| {
            parent.spawn(background::Props {
                file,
                rank,
            }.bundle());
            parent.spawn(labels::Props {
                file,
                rank,
                font,
            }.bundle());
            parent.spawn(EmptyPieceContainerBundle::default());
        });
    }
}

