use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use iyes_perf_ui::prelude::*;

mod player;

use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: bevy::window::PresentMode::Immediate,
                    // mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }).set(ImagePlugin::default_nearest()))
            .add_plugins(PlayerPlugin)
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)

            .add_systems(Startup, setup)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.05, 0.05, 0.05)),
            ..default()
        },
        ..default()
    });

    commands.spawn(PerfUiCompleteBundle::default());
}