use bevy::{input::ButtonInput, prelude::{KeyCode, Query, Res, With}};

use crate::components::{Player, Velocity};

pub fn handle_player_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
	let mut player_vel = query.single_mut();
	const SPEED: f32 = 2.;
	if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
		player_vel.0 = -SPEED;
	} else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
		player_vel.0 = SPEED;
	} else {
		player_vel.0 = 0.;
	}
}