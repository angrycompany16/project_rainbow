use bevy::{math::vec2, prelude::*};

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin  {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::linear_rgb(0.9, 1.0, 1.0),
            custom_size: Some(vec2(100.0, 100.0)),
            ..default()
        },
        ..default()
    })
    .insert(Player);
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction: Vec2 = vec2(0.0, 0.0);

    if input.pressed(KeyCode::KeyW) { direction.y += 1.0; } 
    if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if input.pressed(KeyCode::KeyD) { direction.x += 1.0 } 
    if input.pressed(KeyCode::KeyA) { direction.x -= 1.0 }

    if direction.length_squared() > 0.0 { direction = direction.normalize() }

    let mut player_transform = player_q.get_single_mut().expect("More than one player in scene");

    player_transform.translation += time.delta_seconds() * direction.extend(0.0) * 100.0;
}