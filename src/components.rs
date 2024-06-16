use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Alien;

#[derive(Component)]
pub struct Velocity(pub f32);

#[derive(Component)]
pub struct AnimationIndexes {
	pub start: usize,
	pub end: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);


#[derive(Component)]
pub struct Hitable;

#[derive(Component)]
pub struct Projectile;