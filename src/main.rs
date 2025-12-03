use crate::components::*;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

mod components;
mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), player::plugin))
        .add_systems(Startup, setup)
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
        MovementSpeed(6000.0),
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
        MovementSpeed(6000.0),
        Health(20),
        Damage(10),
        Enemy,
    ));
}

fn chase_player(
    time: Res<Time>,
    enemy_query: Query<(&mut LinearVelocity, &Position, &MovementSpeed), With<Enemy>>,
    player: Single<&Position, (With<Player>, Without<Enemy>)>,
) {
    let delta_time = time.delta_secs();
    let player_position = Vec2::new(player.x, player.y);

    for (mut velocity, position, speed) in enemy_query {
        let enemy_position = Vec2::new(position.x, position.y);
        let direction = (player_position - enemy_position).normalize();

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
