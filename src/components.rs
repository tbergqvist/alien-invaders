use bevy::{math::Vec2, prelude::Component, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Alien;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct AnimationIndexes {
	pub start: usize,
	pub end: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Hitable {
	pub width: f32,
	pub height: f32,
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct FireCooldown(pub Timer);