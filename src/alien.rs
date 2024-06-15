use bevy::{asset::{AssetServer, Assets}, math::{Vec2, Vec3}, prelude::{Commands, Component, Query, Res, ResMut, Resource, With}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Timer, TimerMode}, transform::components::Transform, utils::default};

use crate::animation::{AnimationIndexes, AnimationTimer};

#[derive(Component)]
pub struct Alien;

#[derive(Resource)]
pub struct AlienCollectiveState {
	pub moving_direction: f32,
}

impl AlienCollectiveState {
	pub fn new() -> Self {
		Self {
			moving_direction: 1.,
		}
	}
}

pub fn spawn_aliens(commands: &mut Commands, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
	let alien1_spritesheet = get_spritesheet_component("alien1.png", asset_server, texture_atlas_layouts);
	let alien2_spritesheet = get_spritesheet_component("alien2.png", asset_server, texture_atlas_layouts);
	let alien3_spritesheet = get_spritesheet_component("alien3.png", asset_server, texture_atlas_layouts);

	spawn_specific_aliens(commands, 11, 200., alien1_spritesheet);
	spawn_specific_aliens(commands, 22, 180., alien2_spritesheet);
	spawn_specific_aliens(commands, 22, 140., alien3_spritesheet);
}

fn spawn_specific_aliens(commands: &mut Commands, amounts: i32, start_y: f32, spritesheet: SpriteSheetBundle) {
		for i in 0..amounts {
			let mut sheet = spritesheet.clone();
			let x = i % 11;
			let y = i / 11;
			sheet.transform.translation = Vec3::new(x as f32 * 20., start_y - (y as f32 * 20.), 0.);
			commands.spawn((sheet, Alien, AnimationIndexes { start: 0, end: 1 }, AnimationTimer(Timer::from_seconds(y as f32, TimerMode::Once))));
		}
}

pub fn move_aliens(mut alien_state: ResMut<AlienCollectiveState>, mut query: Query<&mut Transform, With<Alien>>) {
	let mut reached_edge = false;
	let moving_speed = alien_state.moving_direction * 0.1;
	for mut transform in query.iter_mut() {
		let x = transform.translation.x;
		transform.translation.x += moving_speed;
		if (x < 0. && alien_state.moving_direction < 0.) || (x > 280. && alien_state.moving_direction > 0.) {
			reached_edge = true;
		}
	}

	if reached_edge {
		alien_state.moving_direction *= -1.;
		for mut transform in query.iter_mut() {
			transform.translation.y -= 20.;
		}
	}
}

fn get_spritesheet_component(path: &'static str, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) -> SpriteSheetBundle {
	let alien1_texture = asset_server.load(path);
	let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None);
	let texture_atlas_layout = texture_atlas_layouts.add(layout);
	SpriteSheetBundle {
		texture: alien1_texture,
		atlas: TextureAtlas {
			layout: texture_atlas_layout,
			index: 0,
		},
		..default()
	}
}