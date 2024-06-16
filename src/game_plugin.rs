use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, math::Vec2, render::{camera::ClearColor, color::Color}, sprite::TextureAtlasLayout};

use crate::{alien::{create_aliens, move_aliens, AlienCollectiveState}, animation::animate, bundles::{create_camera_bundle, create_player_bundle}, movement::move_entities, player::handle_player_input};

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
	let player_texture = asset_server.load("player.png");
	let alien_textures = vec![asset_server.load("alien1.png"), asset_server.load("alien2.png"), asset_server.load("alien3.png")];
	let player_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 1, None, None));
	let alien_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None));
	
	commands.spawn(create_camera_bundle());
	commands.insert_resource(AlienCollectiveState::new());
	commands.spawn(create_player_bundle(player_texture, player_layout));
	commands.spawn_batch(create_aliens(alien_textures, alien_layout));

}