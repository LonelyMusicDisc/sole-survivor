use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_message::<MovementAction>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_keyboard, movement).chain())
        .add_systems(Update, (despawn_dead_entities, chase_player))
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
        ))
        .observe(on_player_collides_with_enemy);

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
    commands.spawn((
        Text2d::new("E"),
        TextFont {
            font_size: 24.0,
            font: default(),
            ..default()
        },
        TextColor(Color::linear_rgb(1.0, 0.2, 0.0)),
        RigidBody::Dynamic,
        Collider::circle(10.0),
        LinearDamping(100.0),
        LockedAxes::ROTATION_LOCKED,
        MovementSpeed(180.0),
        Health(20),
        Damage(10),
        Enemy,
    ));

    commands.spawn((
        Text2d::new("E"),
        TextFont {
            font_size: 24.0,
            font: default(),
            ..default()
        },
        TextColor(Color::linear_rgb(1.0, 0.2, 0.0)),
        RigidBody::Dynamic,
        Collider::circle(10.0),
        LinearDamping(100.0),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(-15.0, -5.0, 1.0),
        MovementSpeed(180.0),
        Health(20),
        Damage(10),
        Enemy,
    ));
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Damage(i32);

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

fn chase_player(
    time: Res<Time>,
    enemy_query: Query<(&mut LinearVelocity, &Position, &MovementSpeed), With<Enemy>>,
    player: Single<&Position, (With<Player>, Without<Enemy>)>,
) {
    let delta_time = time.delta_secs();

    for (mut velocity, position, speed) in enemy_query {
        let direction = Vec2::new(player.x - position.x, player.y - position.y);

        velocity.x += direction.x * speed.0 * delta_time;
        velocity.y += direction.y * speed.0 * delta_time;
    }
}

fn on_player_collides_with_enemy(
    trigger: On<CollisionStart>,
    enemy_query: Query<&Damage, With<Enemy>>,
    mut health: Single<&mut Health, With<Player>>,
) {
    let entity = trigger.collider2;

    if !enemy_query.contains(entity) {
        return;
    };
    let Ok(damage) = enemy_query.get(entity) else {
        warn!("Failed to get component");
        return;
    };
    health.0 -= damage.0;
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.0 > 0 {
            return;
        };
        commands.entity(entity).despawn();
    }
}
