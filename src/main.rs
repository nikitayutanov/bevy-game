use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .run();
}
