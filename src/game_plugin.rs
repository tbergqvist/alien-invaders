use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, math::{Vec2, Vec3}, prelude::Camera2dBundle, render::{camera::{ClearColor, ScalingMode}, color::Color}, sprite::TextureAtlasLayout, transform::components::Transform};

use crate::{alien::{move_aliens, spawn_aliens, AlienCollectiveState}, animation::animate, movement::move_entities, player::{handle_player_input, spawn_player}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, create_world)
			.add_systems(FixedUpdate, (move_entities, move_aliens))
			.add_systems(Update, (handle_player_input, animate))
		;
	}
}

fn create_world(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	spawn_camera(&mut commands);
	spawn_player(&mut commands, &asset_server, &mut texture_atlas_layouts);
	spawn_aliens(&mut commands, &asset_server, &mut texture_atlas_layouts);
}

fn spawn_camera(commands: &mut Commands) {
	let mut camera = Camera2dBundle::default();
	camera.transform = Transform::from_translation(Vec3::new(140., 100., 1.));
	camera.projection.scaling_mode = ScalingMode::WindowSize(4.);
	commands.spawn(camera);
	commands.insert_resource(AlienCollectiveState::new());
}