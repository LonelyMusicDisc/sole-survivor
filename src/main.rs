use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_message::<MovementAction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_keyboard, movement).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera2d,
            Text2d::new("@"),
            TextFont {
                font_size: 24.0,
                font: default(),
                ..default()
            },
            TextColor(Color::WHITE),
            RigidBody::Dynamic,
            CollisionEventsEnabled,
            Collider::circle(10.0),
            LockedAxes::ROTATION_LOCKED,
            Transform::from_xyz(200.0, -50.0, 1.0),
            MovementSpeed(8000.0),
            Health(100),
            Player,
        ));

    commands.spawn((
        Text2d::new("#"),
        TextFont {
            font_size: 32.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        RigidBody::Static,
        Collider::rectangle(32.0, 32.0),
        Transform::from_xyz(0.0, 50.0, 1.0),
    ));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct MovementSpeed(f32);

#[derive(Message, Debug, Reflect)]
enum MovementAction {
    Move(Vec2),
}

fn handle_keyboard(
    mut messages: MessageWriter<MovementAction>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    let left = keycode.pressed(KeyCode::KeyA);
    let right = keycode.pressed(KeyCode::KeyD);
    let up = keycode.pressed(KeyCode::KeyW);
    let down = keycode.pressed(KeyCode::KeyS);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as Scalar, vertical as Scalar);
    if direction == Vec2::ZERO {
        return;
    };
    messages.write(MovementAction::Move(direction));
}

fn movement(
    time: Res<Time>,
    mut movement_messages: MessageReader<MovementAction>,
    player: Single<(&mut LinearVelocity, &MovementSpeed), With<Player>>,
) {
    let (mut linear_velocity, speed) = player.into_inner();
    let delta_time = time.delta_secs();

    linear_velocity.x = 0.0;
    linear_velocity.y = 0.0;

    for message in movement_messages.read() {
        match message {
            MovementAction::Move(direction) => {
                linear_velocity.x += direction.x * speed.0 * delta_time;
                linear_velocity.y += direction.y * speed.0 * delta_time;
            }
        }
    }
}
