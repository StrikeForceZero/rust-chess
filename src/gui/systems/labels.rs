use bevy::prelude::*;
use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;

use super::coloring::SquareColor;

#[derive(Clone, Debug)]
pub struct Props {
    pub file: BoardFile,
    pub rank: BoardRank,
    pub font: Handle<Font>,
}

impl Props {
    pub fn bundle(self) -> impl Bundle {
        let Props {
            file,
            rank,
            font,
        } = self;
        let color = SquareColor::from_file_rank(file, rank).as_inverse().as_color();
        let text = format!("{}{}", file.as_char(), rank.as_usize());
        let font_size = 25.;
        let style = TextStyle {
            font,
            font_size,
            color,
            ..default()
        };
        let transform = Transform::from_translation(Vec3::new(
            -font_size - 10.,
            -font_size - 15.,
            1.,
        ));
        let text = Text::from_section(text, style)
            .with_alignment(TextAlignment::Center);
        Text2dBundle {
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            text,
            transform,
            ..default()
        }
    }
}
