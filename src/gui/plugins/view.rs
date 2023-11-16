use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct Plugin;

pub const SQUARE_SIZE: f32 = 100.;
pub const WIDTH: f32 = SQUARE_SIZE * 8.;
pub const HEIGHT: f32 = SQUARE_SIZE * 8.;
pub const CAMERA_OFFSET_Y: f32 = 50.;
pub const CAMERA_OFFSET_X: f32 = CAMERA_OFFSET_Y;

#[derive(Component, Copy, Clone)]
pub struct MainCamera;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Chess".to_string(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        // present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
            )
            .add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            )
            .add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn(
        (
            Camera2dBundle {
                transform: Transform::from_xyz(
                    WIDTH / 2. + CAMERA_OFFSET_X,
                    HEIGHT / 2. + CAMERA_OFFSET_Y,
                    100.
                ),
                ..default()
            },
            MainCamera,
        )
    );
}
