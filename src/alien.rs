use std::collections::HashMap;

use bevy::{asset::Handle, math::Vec2, prelude::{Bundle, Commands, Query, Res, With}, render::texture::Image, sprite::TextureAtlasLayout, time::Time, transform::components::{GlobalTransform, Transform}};

use crate::{bundles::{create_alien_bundle, create_alien_projectile}, components::Alien, resources::{AlienCollective, AssetStore}};

pub fn create_aliens(textures: Vec<Handle<Image>>, layout: Handle<TextureAtlasLayout>) -> Vec<impl Bundle> {
	vec![create_specific_aliens( 11, 200., textures[0].clone(), layout.clone()),
		create_specific_aliens( 22, 180., textures[1].clone(), layout.clone()),
		create_specific_aliens( 22, 140., textures[2].clone(), layout)
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

pub fn move_aliens(mut query: Query<(&mut Transform, &mut AlienCollective)>) {
	let (mut transform, mut alien_state) = query.single_mut();

	let moving_speed = alien_state.moving_direction * 0.1;
	let x = transform.translation.x + moving_speed;
	transform.translation.x = x;
	if (x < 0. && alien_state.moving_direction < 0.) || (x > 80. && alien_state.moving_direction > 0.) {
		alien_state.moving_direction *= -1.;
		transform.translation.y -= 15.;
	}
}

pub fn alien_fire(
	mut commands: Commands,
	time: Res<Time>,
	asset_store: Res<AssetStore>,
	mut alien_query: Query<&mut AlienCollective>,
	query: Query<&GlobalTransform, With<Alien>>
) {
	let mut alien_state = alien_query.single_mut();
	alien_state.shoot_timer.tick(time.delta());

	if !alien_state.shoot_timer.finished() {
		return;
	}
	let positions = query.iter()
		.map(|t| (t.translation().x, t.translation().y))
		.collect::<Vec<_>>();
	
	if positions.len() == 0 {
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

	if valid_positions.len() > 0 {
		let firing_alien = rand::random::<usize>() % valid_positions.len();
		let pos = valid_positions[firing_alien];
		commands.spawn(create_alien_projectile(
			Vec2::new(pos.0 as f32, pos.1 - 20.), 
			asset_store.player_projectile_texture.clone(),
			asset_store.projectile_atlas.clone()
		));
	}
}