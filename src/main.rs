mod camera;
mod player;
mod world;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            CameraPlugin,
            PlayerPlugin,
            WorldPlugin,
        ))
        .run();
}
