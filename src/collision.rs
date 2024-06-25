use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::{Commands, Entity, Query, Res, With}, time::Time, transform::components::Transform};

use crate::{bundles::create_explosion, components::{Collided, Health, Hitable, LifeTimer, Projectile}, resources::AssetStore};

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
			commands.entity(entity2).insert(Collided);
			commands.entity(entity).insert(Collided);
		}
	}
}

pub fn decrease_health(
	mut commands: Commands,
	asset_store: Res<AssetStore>,
	mut collided: Query<(Entity, &mut Health, &Transform), With<Collided>>
) {
	for (entity, mut health, transform) in collided.iter_mut() {
		health.hp -= 1;
		if health.hp <= 0 {
			commands.entity(entity).despawn();
				commands.spawn(create_explosion(transform.translation.truncate(), asset_store.explosion_texture.clone(), asset_store.explosion_atlas.clone()));
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

pub fn kill_old_entities(
	mut commands: Commands,
	time: Res<Time>,
	mut life_timers: Query<(Entity, &mut LifeTimer)>
) {
	for (entity, mut life_timer) in life_timers.iter_mut() {
		life_timer.timer.tick(time.delta());
		if life_timer.timer.finished() {
			commands.entity(entity).despawn();
		}
	}
}