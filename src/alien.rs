use std::collections::HashMap;

use bevy::{asset::Handle, math::Vec2, prelude::{Bundle, Commands, Query, Res, ResMut, With}, render::texture::Image, sprite::TextureAtlasLayout, time::Time, transform::components::Transform};

use crate::{bundles::{create_alien_bundle, create_alien_projectile}, components::Alien, resources::{AlienCollectiveState, AssetStore}};

pub fn create_aliens(textures: Vec<Handle<Image>>, layout: Handle<TextureAtlasLayout>) -> Vec<impl Bundle> {
	vec![create_specific_aliens( 11, 250., textures[0].clone(), layout.clone()),
		create_specific_aliens( 22, 230., textures[1].clone(), layout.clone()),
		create_specific_aliens( 22, 190., textures[2].clone(), layout)
	]
	.into_iter()
	.flatten()
	.collect()
}

fn create_specific_aliens(amounts: i32, start_y: f32, texture: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> Vec<impl Bundle> {
	(0..amounts).map(|i| {
		let x = i % 11;
		let y = i / 11;

		let position = Vec2::new(x as f32 * 20., start_y - (y as f32 * 20.));
		create_alien_bundle(position, y as usize, texture.clone(), layout.clone())
	})
	.collect()
}

pub fn move_aliens(mut alien_state: ResMut<AlienCollectiveState>, mut query: Query<&mut Transform, With<Alien>>) {
	let mut reached_edge = false;
	let aliens_alive = query.iter().count() as f32;
	let moving_speed = alien_state.moving_direction * (1. / aliens_alive) * 5.;
	for mut transform in query.iter_mut() {
		let x = transform.translation.x;
		transform.translation.x += moving_speed;
		if (x < 0. && alien_state.moving_direction < 0.) || (x > 360. && alien_state.moving_direction > 0.) {
			reached_edge = true;
		}
	}

	if reached_edge {
		alien_state.moving_direction *= -1.;
		for mut transform in query.iter_mut() {
			transform.translation.y -= 15.;
		}
	}
}

pub fn alien_fire(
	mut commands: Commands,
	time: Res<Time>,
	asset_store: Res<AssetStore>,
	mut alien_state: ResMut<AlienCollectiveState>,
	query: Query<&Transform, With<Alien>>
) {
	alien_state.shoot_timer.tick(time.delta());

	if !alien_state.shoot_timer.finished() {
		return;
	}
	let positions = query.iter()
		.map(|t| (t.translation.x, t.translation.y))
		.collect::<Vec<_>>();
	
	if positions.is_empty() {
		return;
	}

	let valid_positions = positions.iter().fold(HashMap::new(), |mut map, (x, y)| {
		map.entry(*x as i32)
			.and_modify(|min_y| *min_y = y.min(*min_y))
			.or_insert(*y);
		map
	})
	.into_iter()
	.collect::<Vec<_>>();

	if !valid_positions.is_empty() {
		let firing_alien = rand::random::<usize>() % valid_positions.len();
		let pos = valid_positions[firing_alien];
		commands.spawn(create_alien_projectile(
			Vec2::new(pos.0 as f32, pos.1 - 20.), 
			asset_store.player_projectile_texture.clone(),
			asset_store.projectile_atlas.clone()
		));
	}
}