use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Message, Debug, Reflect)]
pub enum MovementAction {
    Move(Vec2),
}
