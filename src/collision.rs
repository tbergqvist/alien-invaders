use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::{Commands, Entity, Query}, transform::components::Transform};

use crate::components::{Hitable, Projectile};

pub fn check_collisions(
		mut commands: Commands,
		hitables: Query<(Entity, &Transform, &Hitable, Option<&Projectile>)>
) {
	let mut iter = hitables.iter_combinations();

	while let Some([(entity, transform, hitable, projectile), (entity2, transform2, hitable2, projectile2)]) = iter.fetch_next()  {
		if projectile.is_none() && projectile2.is_none() {
			continue;
		}

		let hitable_aabb = Aabb2d::new(
			transform.translation.truncate(), 
			hitable.size / 2.
		);

		let projectile_aabb = Aabb2d::new(
			transform2.translation.truncate(), 
			hitable2.size / 2.
		);

		if projectile_aabb.intersects(&hitable_aabb) {
			commands.entity(entity2).despawn();
			commands.entity(entity).despawn();
		}
	}
}