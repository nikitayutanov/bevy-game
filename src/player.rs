use bevy::prelude::*;

const PLAYER_SPEED: f32 = 2.5;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
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

    commands.spawn((mesh, material, transform, Player));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player_transform.translation +=
        direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
}
