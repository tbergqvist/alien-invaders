use bevy::{asset::Handle, math::Vec3, prelude::{Bundle, Query, ResMut, Resource, With}, render::texture::Image, sprite::TextureAtlasLayout, transform::components::Transform};

use crate::{bundles::create_alien_bundle, components::Alien};

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

			let position = Vec3::new(x as f32 * 20., start_y - (y as f32 * 20.), 0.);
			create_alien_bundle(position, y, texture.clone(), layout.clone())
		})
		.collect()
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