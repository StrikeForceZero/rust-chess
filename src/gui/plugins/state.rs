use bevy::prelude::*;
use crate::board::board::Board;
use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;
use crate::state::game_state::GameState;

pub struct Plugin;

#[derive(Resource, Default)]
pub struct CurrentGame {
    game_state: GameState,
    selected: Option<(BoardRank, BoardFile)>,
}

impl CurrentGame {
    pub fn state(&self) -> &GameState {
        &self.game_state
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }

    pub fn board(&self) -> &Board {
        &self.game_state.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.game_state.board
    }

    pub fn selected(&self) -> Option<(BoardRank, BoardFile)> {
        self.selected
    }

    pub fn set_selected(&mut self, rank: BoardRank, file: BoardFile) {
        self.selected = Some((rank, file));
    }

    pub fn clear_selected(&mut self) {
        self.selected = None;
    }
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            // use this if we decide that Default should be a new game state, not empty
            // .init_resource::<CurrentGame>();
        ;
    }
}

fn setup(mut commands: Commands) {
    let state = CurrentGame {
        game_state: GameState::new(),
        ..default()
    };
    commands.insert_resource(state);
}
