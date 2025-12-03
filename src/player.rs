use crate::Scalar;
use crate::components::*;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_keyboard, player_movement).chain())
        .add_message::<MovementAction>();
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

fn player_movement(
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
