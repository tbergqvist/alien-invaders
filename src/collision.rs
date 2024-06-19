use bevy::{prelude::{Commands, Entity, Query, With, Without}, transform::components::Transform};

use crate::components::{Hitable, Projectile};

pub fn check_collisions(
		mut commands: Commands,
		hitables: Query<(Entity, &Transform, &Hitable), Without<Projectile>>,
		projectile: Query<(Entity, &Transform, &Hitable), With<Projectile>>
) {
	for (projectile_entity, projectile_transform, projectile_hitable) in &mut projectile.iter() {
		for (hitable_entity, hitable_transform, hitable) in &mut hitables.iter() {
			if projectile_transform.translation.x < hitable_transform.translation.x + hitable.width
				&& projectile_transform.translation.x + projectile_hitable.width > hitable_transform.translation.x
				&& projectile_transform.translation.y < hitable_transform.translation.y + hitable.height
				&& projectile_transform.translation.y + projectile_hitable.height > hitable_transform.translation.y
			{
				//commands.entity(projectile_entity).despawn();
				//commands.entity(hitable_entity).despawn();
				println!("Collision detected!");
			}
		}
	}
}