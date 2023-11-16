use bevy::prelude::*;
use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;

use super::coloring;
use super::super::view;
use super::super::clicks;

#[derive(Component, Default, Copy, Clone)]
struct Marker;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                update_background
                    .after(coloring::Calculated)
            );
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub file: BoardFile,
    pub rank: BoardRank,
}

impl Props {
    pub fn bundle(self) -> impl Bundle {
        let Props {
            file,
            rank,
        } = self;
        let color = coloring::SquareColor::from_file_rank(file, rank);
        let sprite = {
            let color = color.as_color();
            let custom_size = Some(Vec2::new(view::SQUARE_SIZE, view::SQUARE_SIZE));
            Sprite {
                color,
                custom_size,
                ..default()
            }
        };
        (
            SpriteBundle {
                sprite,
                ..default()
            },
            Marker,
            rank,
            file,
            clicks::Clicked(false),
            color,
        )
    }
}

fn update_background(
    mut q_background: Query<
        (&coloring::SquareColor, &mut Sprite),
        (With<Marker>, Changed<coloring::SquareColor>),
    >,
) {
    for (color, mut sprite) in q_background.iter_mut() {
        sprite.color = color.as_color();
    }
}
