use bevy::{asset::{AssetServer, Assets}, input::ButtonInput, math::Vec2, prelude::{Commands, Component, KeyCode, Query, Res, ResMut, With}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, transform::components::Transform, utils::default};

use crate::movement::Velocity;

#[derive(Component)]
pub struct Player;

pub fn handle_player_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
	let mut player_vel = query.single_mut();
	const SPEED: f32 = 2.;
	if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
		player_vel.0 = -SPEED;
	} else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
		player_vel.0 = SPEED;
	} else {
		player_vel.0 = 0.;
	}
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
	let player_texture = asset_server.load("player.png");
	let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 1, None, None);
	let texture_atlas_layout = texture_atlas_layouts.add(layout);
	
	commands.spawn((
		SpriteSheetBundle {
			texture: player_texture,
			atlas: TextureAtlas {
				layout: texture_atlas_layout,
				index: 0,
			},
			transform: Transform::from_xyz(5., 0., 0.),
			..default()
		},
		Velocity(0.),
		Player,
	));
}