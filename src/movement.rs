use bevy::{prelude::Query, transform::components::Transform};

use crate::components::Velocity;

pub fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		let new_pos_x = transform.translation.x + velocity.0.x;
		let new_pos_y = transform.translation.y + velocity.0.y;
		transform.translation.x = new_pos_x.clamp(0., 360.);
		transform.translation.y = new_pos_y;
	}
}