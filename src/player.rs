use bevy::{input::ButtonInput, math::Vec2, prelude::{Commands, KeyCode, Query, Res, With}, time::Time, transform::components::Transform};

use crate::{bundles::create_player_projectile, components::{FireCooldown, Player, Velocity}, resources::AssetStore};

pub fn handle_player_input(
	mut commands: Commands,
	time: Res<Time>,
	asset_store: Res<AssetStore>,
	input: Res<ButtonInput<KeyCode>>,
	mut player_query: Query<(&mut Velocity, &Transform, &mut FireCooldown), With<Player>>
) {
	let player_res = player_query.get_single_mut();
	if let Ok((mut player_velocity, player_transform, mut fire_cooldown)) = player_res {
		const SPEED: f32 = 2.;
		if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
			player_velocity.0.x = -SPEED;
		} else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
			player_velocity.0.x = SPEED;
		} else {
			player_velocity.0.x = 0.;
		}

		fire_cooldown.0.tick(time.delta());
		if input.pressed(KeyCode::Space) && fire_cooldown.0.finished() {
			let projectile_pos = Vec2::new(player_transform.translation.x, player_transform.translation.y + 8.);
			commands.spawn(create_player_projectile(projectile_pos, asset_store.player_projectile_texture.clone(), asset_store.projectile_atlas.clone()));
			fire_cooldown.0.reset();
		}
	}
}