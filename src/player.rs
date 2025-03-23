use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)));

    let color = Color::srgb_u8(184, 117, 83);
    let material = MeshMaterial3d(materials.add(color));

    let transform = Transform::from_xyz(0.0, 0.5, 0.0);

    commands.spawn((mesh, material, transform));
}
