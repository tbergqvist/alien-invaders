use bevy::{prelude::{Component, Query}, transform::components::Transform};

#[derive(Component)]
pub struct Velocity(pub f32);

pub fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		let new_pos = transform.translation.x + velocity.0;
		transform.translation.x = new_pos.clamp(0., 280.);
	}
}