use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3d::default();
    let transform = Transform::from_xyz(5.0, 4.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn((camera, transform));
}
