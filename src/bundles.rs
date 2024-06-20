use bevy::{asset::Handle, math::{Vec2, Vec3}, prelude::{Bundle, Camera2dBundle}, render::{camera::ScalingMode, texture::Image}, sprite::{Anchor, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::components::{Alien, AnimationIndexes, AnimationTimer, FireCooldown, Hitable, Player, Projectile, Velocity};

pub fn create_camera_bundle() -> impl Bundle {
	let mut camera = Camera2dBundle::default();
	camera.transform = Transform::from_translation(Vec3::new(140., 100., 1.));
	camera.projection.scaling_mode = ScalingMode::WindowSize(4.);
	camera
}

pub fn create_player_bundle(texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	(
		SpriteSheetBundle {
			texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: 0,
			},
			transform: Transform::from_xyz(5., 0., 0.),
			..default()
		},
		Velocity(Vec2::ZERO),
		FireCooldown(Timer::from_seconds(1., TimerMode::Once)),
		Player,
	)
}

pub fn create_alien_bundle(location: Vec3, start_frame: usize, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	(
		SpriteSheetBundle {
			texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: start_frame as usize,
			},
			transform: Transform::from_translation(location),
			..default()
		}, 
		Alien, 
		AnimationIndexes { start: 0, end: 1 }, 
		AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
		Hitable { size: Vec2::new(16., 12.) },
	)
}

pub fn create_player_projectile(location: Vec3, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	(
		SpriteSheetBundle {
			texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: 0,
			},
			transform: Transform::from_translation(location),
			..default()
		},
		Velocity(Vec2{ x: 0., y: 2. }),
		Hitable { size: Vec2::new(3., 10.) },
		AnimationIndexes { start: 0, end: 1 },
		AnimationTimer(Timer::from_seconds(0.01, TimerMode::Repeating)),
		Projectile,
	)
}