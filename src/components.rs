use bevy::{math::Vec2, prelude::Component, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Alien;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Animated {
	pub start: usize,
	pub end: usize,
	pub timer: Timer
}

#[derive(Component)]
pub struct Hitable {
	pub size: Vec2
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct FireCooldown(pub Timer);