use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::view::RenderLayers};
use iyes_perf_ui::prelude::*;

mod player;
mod camera;

use player::PlayerPlugin;
use camera::PixelCameraPlugin;

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
            .add_plugins(PixelCameraPlugin {
                pixel_scale_factor: 4,
                screen_width: 1920,
                screen_height: 1080,
            })

            .add_systems(Startup, setup)
        ;
    }
}

fn setup(
    mut commands: Commands,
) {
    
    commands.spawn(PerfUiCompleteBundle::default());
}