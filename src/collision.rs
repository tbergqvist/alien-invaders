use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::{Commands, Entity, Query, With}, transform::components::{GlobalTransform, Transform}};

use crate::components::{Collided, Health, Hitable, Projectile};

pub fn check_collisions(
		mut commands: Commands,
		hitables: Query<(Entity, &GlobalTransform, &Hitable, Option<&Projectile>)>
) {
	let mut iter = hitables.iter_combinations();

	while let Some([(entity, transform, hitable, projectile), (entity2, transform2, hitable2, projectile2)]) = iter.fetch_next()  {
		if projectile.is_none() && projectile2.is_none() {
			continue;
		}

		let hitable_aabb = Aabb2d::new(
			transform.translation().truncate(), 
			hitable.size / 2.
		);

		let projectile_aabb = Aabb2d::new(
			transform2.translation().truncate(), 
			hitable2.size / 2.
		);

		if projectile_aabb.intersects(&hitable_aabb) {
			commands.entity(entity2).insert(Collided);
			commands.entity(entity).insert(Collided);
		}
	}
}

pub fn decrease_health(
		mut commands: Commands,
		mut collided: Query<(Entity, &mut Health, &Collided)>
) {
	for (entity, mut health, _) in collided.iter_mut() {
		health.hp -= 1;
		if health.hp <= 0 {
			commands.entity(entity).despawn();
		}
	}
}

pub fn remove_collided(
		mut commands: Commands,
		collided: Query<Entity, With<Collided>>
) {
	for entity in collided.iter() {
		commands.entity(entity).remove::<Collided>();
	}
}