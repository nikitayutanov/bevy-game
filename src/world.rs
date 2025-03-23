use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_light));
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0)));

    let color = Color::srgb_u8(91, 103, 12);
    let material = MeshMaterial3d(materials.add(color));

    commands.spawn((mesh, material));
}

fn spawn_light(mut commands: Commands) {
    let light = PointLight {
        shadows_enabled: true,
        ..default()
    };

    let tranform = Transform::from_xyz(0.0, 5.0, 0.0);

    commands.spawn((light, tranform));
}
