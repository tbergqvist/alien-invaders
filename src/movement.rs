use bevy::{prelude::Query, transform::components::Transform};

use crate::components::Velocity;

pub fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		let new_pos = transform.translation.x + velocity.0;
		transform.translation.x = new_pos.clamp(0., 280.);
	}
}