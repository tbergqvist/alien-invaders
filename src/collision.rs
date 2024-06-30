use bevy::{asset::{Assets, Handle}, math::{bounding::Aabb2d, Vec2}, prelude::{Commands, Entity, Query, Res, With}, render::texture::Image, time::Time, transform::components::Transform};

use crate::{bundles::create_explosion, components::{Collided, Health, Hitable, LifeTimer, Projectile}, resources::AssetStore};

pub fn check_collisions(
		mut commands: Commands,
		hitables: Query<(Entity, &Transform, &Hitable, &Handle<Image>, Option<&Projectile>)>,
		image_assets: Res<Assets<Image>>
) {
	let mut iter = hitables.iter_combinations();

	while let Some([(entity, transform, hitable, image, _), (entity2, transform2, hitable2, _, projectile2)]) = iter.fetch_next()  {
		if projectile2.is_none() {
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

		let image_a = image_assets.get(image).unwrap();

		if let Some(intersection) = intersect(&hitable_aabb, &projectile_aabb) {
			let intersection_min = intersection.min;
			let intersection_max = intersection.max;

			let a_offset = transform.translation.truncate() - hitable.size / 2.;

			if pixel_collission(intersection_min, intersection_max, image_a, a_offset, &hitable.size) {
				commands.entity(entity2).insert(Collided);
				commands.entity(entity).insert(Collided);
				continue;
			}
		}
	}
}

fn pixel_collission(intersection_min: Vec2, intersection_max: Vec2, image_a: &Image, a_offset: Vec2, size: &Vec2) -> bool {
	for y in intersection_min.y as i32..=intersection_max.y as i32 {
		for x in intersection_min.x as i32..=intersection_max.x as i32 {
			let mut y_val = ((size.y as i32 - 1) - ((y - a_offset.y as i32) as i32)) * 4;
			y_val = y_val.max(0).min((size.y as i32 - 1) * 4);
			let mut x_val = (x - a_offset.x as i32) as i32 * 4;
			x_val = x_val.max(0).min((size.x as i32 - 1) * 4);
			let a_index = (y_val * size.x as i32 + x_val) as usize;
			let a_alpha = image_a.data[a_index + 3];
			if a_alpha > 0 {
				return true;
			}
		}
	}
	false
}

fn intersect(aabb1: &Aabb2d, aabb2: &Aabb2d) -> Option<Aabb2d> {
	let min_x = aabb1.min.x.max(aabb2.min.x);
	let min_y = aabb1.min.y.max(aabb2.min.y);
	let max_x = aabb1.max.x.min(aabb2.max.x);
	let max_y = aabb1.max.y.min(aabb2.max.y);

	if min_x <= max_x && min_y <= max_y {
		Some(Aabb2d { 
			min: Vec2::new(min_x, min_y), 
			max: Vec2::new(max_x, max_y) 
		})
	} else {
		None
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