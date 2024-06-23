use bevy::{asset::Handle, math::{Vec2, Vec3}, prelude::{Bundle, Camera2dBundle}, render::{camera::{OrthographicProjection, ScalingMode}, texture::Image}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::components::{Alien, Animated, FireCooldown, Health, Hitable, Player, Projectile, Velocity};

pub fn create_camera_bundle() -> impl Bundle {
	Camera2dBundle {
		transform: Transform::from_translation(Vec3::new(180., 100., 1.)),
		projection: OrthographicProjection {
			scaling_mode: ScalingMode::WindowSize(3.),
			..default()
		},
		..default()
	}
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
		Hitable { size: Vec2::new(16., 4.) },
		Player,
		Health { hp: 3 },
	)
}

pub fn create_alien_bundle(location: Vec2, start_frame: usize, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	(
		SpriteSheetBundle {
			texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: start_frame,
			},
			transform: Transform::from_translation(location.extend(0.)),
			..default()
		}, 
		Alien, 
		Animated { start: 0, end: 1, timer: Timer::from_seconds(1., TimerMode::Repeating) }, 
		Hitable { size: Vec2::new(16., 12.) },
		Health { hp: 1 },
	)
}

pub fn create_player_projectile(location: Vec2, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	create_projectile(location, texture, texture_atlas_layout, Vec2{ x: 0., y: 6. })
}

pub fn create_alien_projectile(location: Vec2, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
	create_projectile(location, texture, texture_atlas_layout, Vec2{ x: 0., y: -2. })
}

fn create_projectile(location: Vec2, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>, velocity: Vec2) -> impl Bundle {
	(
		SpriteSheetBundle {
			texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: 0,
			},
			transform: Transform::from_translation(location.extend(0.)),
			..default()
		},
		Velocity(velocity),
		Hitable { size: Vec2::new(1., 8.) },
		Animated { start: 0, end: 3, timer: Timer::from_seconds(1. / 60., TimerMode::Repeating) },
		Projectile,
		Health { hp: 1 },
	)
}