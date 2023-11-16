use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use smallvec::SmallVec;
use crate::board::board_file::BoardFile;
use crate::board::board_rank::BoardRank;
use crate::gui::plugins::view::{CAMERA_OFFSET_X, CAMERA_OFFSET_Y};
use super::view;

pub struct Plugin;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Clicked(pub bool);

impl Clicked {
    pub fn is_clicked(&self) -> bool {
        self.0
    }

    pub fn set_clicked(&mut self, value: bool) {
        self.0 = value
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct Calculated;

#[derive(Debug, Default, bevy::prelude::Event)]
struct Event(SmallVec<[Entity; 3]>);

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Event>()
            .add_systems(
                Update,
                mouse_button_input
                    .in_set(Calculated)
            )
            .add_systems(
                Update,
                update_clicked
                    .after(mouse_button_input)
                    .in_set(Calculated)
            );
    }
}

fn mouse_button_input(
    // need to get window dimensions
    wnds: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<view::MainCamera>>,
    mut ev_click: EventWriter<Event>,
    space: Query<(&BoardRank, &BoardFile, Entity)>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let Ok(wnd) = wnds.get_single() else {
        return;
    };

    // check if the cursor is inside the window and get its position
    let Some(screen_pos) = wnd.cursor_position() else {
        return;
    };
    // get the size of the window
    let window_size = Vec2::new(wnd.width(), wnd.height());

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    const MAX_RANK: u8 = 8;
    let x = world_pos.x.round() + CAMERA_OFFSET_X;
    let y = world_pos.y.round() + CAMERA_OFFSET_Y;
    let raw_file = (x / view::SQUARE_SIZE).trunc() as u8;
    // coord system is 0,0 top left so we need to invert the y to match the rank order
    let raw_rank = MAX_RANK + 1 - (y / view::SQUARE_SIZE).trunc() as u8;
    if !(1..=MAX_RANK).contains(&raw_file) || !(1..=MAX_RANK).contains(&raw_rank)
    {
        info!("Clear click");
        ev_click.send(Event::default());
        return;
    }
    let Ok(click_file) = BoardFile::from_u8(raw_file) else {
        warn!("failed to get file from {raw_file}");
        return;
    };
    let Ok(click_rank) = BoardRank::from_u8(raw_rank) else {
        warn!("failed to get rank from {raw_rank}");
        return;
    };

    let mut entities = SmallVec::new();
    for (&rank, &file, entity) in space.iter() {
        if click_file != file || click_rank != rank {
            continue;
        }
        entities.push(entity);
    }
    info!(file = ?click_file, rank = ?click_rank, ?entities, "Emit click");
    ev_click.send(Event(entities))
}

fn update_clicked(
    mut q_background: Query<
        (Entity, &mut Clicked),
        (With<BoardRank>, With<BoardFile>),
    >,
    mut ev_click: EventReader<Event>,
) {
    for Event(clicked_entities) in ev_click.read() {
        debug!(?clicked_entities, "Click-received");
        for (entity, mut clicked) in q_background.iter_mut() {
            debug!(?entity, ?clicked, "Checking entity for click");
            if clicked_entities.contains(&entity) {
                // if we don't allow clicked to be set it will prevent the player
                // from moving their last moved piece after a bot has moved
                // until they select a new piece
                /*if clicked.is_clicked() {
                    continue;
                }*/
                info!(?clicked, ?entity, "Update to clicked");
                clicked.set_clicked(true)
            } else {
                /*if !clicked.is_clicked() {
                    continue;
                }*/
                info!(?clicked, ?entity, "Update to unclicked");
                clicked.set_clicked(false)
            }
        }
    }
}
