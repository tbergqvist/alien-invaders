use bevy::{asset::Handle, math::Vec3, prelude::{Bundle, Camera2dBundle}, render::{camera::ScalingMode, texture::Image}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::components::{Alien, Player, Velocity, AnimationIndexes, AnimationTimer};

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
		Velocity(0.),
		Player,
	)
}

pub fn create_alien_bundle(location: Vec3, timer_start: i32, texture: Handle<Image>, texture_atlas_layout: Handle<TextureAtlasLayout>) -> impl Bundle {
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
		Alien, 
		AnimationIndexes { start: 0, end: 1 }, 
		AnimationTimer(Timer::from_seconds(timer_start as f32, TimerMode::Once))
	)
}