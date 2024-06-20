use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::{Commands, Entity, Query, With, Without}, transform::components::Transform};

use crate::components::{Hitable, Projectile};

pub fn check_collisions(
		mut commands: Commands,
		hitables: Query<(Entity, &Transform, &Hitable), Without<Projectile>>,
		projectile: Query<(Entity, &Transform, &Hitable), With<Projectile>>
) {
	for (projectile_entity, projectile_transform, projectile_hitable) in &mut projectile.iter() {
		let projectile_aabb = Aabb2d::new(
			projectile_transform.translation.truncate(), 
			projectile_hitable.size / 2.
		);
		
		for (hitable_entity, hitable_transform, hitable) in &mut hitables.iter() {
			let hitable_aabb = Aabb2d::new(
				hitable_transform.translation.truncate(), 
				hitable.size / 2.
			);

			if projectile_aabb.intersects(&hitable_aabb) {
				commands.entity(projectile_entity).despawn();
				commands.entity(hitable_entity).despawn();
			}
		}
	}
}