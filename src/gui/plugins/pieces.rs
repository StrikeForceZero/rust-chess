use bevy::prelude::*;
use crate::board::board_file::BoardFile;
use crate::board::board_position::BoardPosition;
use crate::board::board_rank::BoardRank;
use crate::gui::plugins::board::PieceContainer;
use crate::gui::systems::placed::PlacedPiece;

use super::state;
use crate::gui::systems::sprites::SpriteMapResource;

pub struct Plugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemSet)]
pub struct Render;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                update_pieces
                    .in_set(Render)
            )
            .add_systems(
                Update,
                update_sprite
                    .after(update_pieces)
                    .in_set(Render)
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(SpriteMapResource::new(&asset_server));
}

fn update_pieces(
    current: Res<state::CurrentGame>,
    mut q_square: Query<(&BoardRank, &BoardFile, &mut PlacedPiece)>,
) {
    for (&rank, &file, mut placed) in q_square.iter_mut() {
        let pos = BoardPosition::from(file, rank);
        let actual = PlacedPiece::from_chess_piece(current.board().get(pos));
        if actual == *placed {
            continue;
        }
        *placed = actual;
    }
}

fn update_sprite(
    mut commands: Commands,
    sprite_map_res: Res<SpriteMapResource>,
    q_square: Query<(&PlacedPiece, Entity, &Children), Changed<PlacedPiece>>,
    q_piece_containers: Query<Entity, With<PieceContainer>>,
) {
    for (&piece, square_entity, children) in q_square.iter() {
        // Despawn only the sprite entities associated with the PieceContainer
        for &child in children.iter() {
            if q_piece_containers.get(child).is_ok() {
                commands.entity(child).despawn_recursive();
            }
        }

        // If there's a texture for the new piece, spawn a new sprite
        if let Some(texture) = sprite_map_res.image_from_piece(piece) {
            commands.entity(square_entity).with_children(|parent| {
                parent.spawn(SpriteBundle {
                    // make sure the sprites are displayed on top
                    transform: Transform::from_xyz(0., 0., 1.),
                    texture,
                    ..default()
                }).insert(PieceContainer);
            });
        }
    }
}
