use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, render::{camera::ClearColor, color::Color}, sprite::TextureAtlasLayout};

use crate::{alien::{create_aliens, move_aliens, AlienCollectiveState}, animation::animate, asset_store::AssetStore, bundles::{create_camera_bundle, create_player_bundle}, collision::check_collisions, movement::move_entities, player::handle_player_input};

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, create_world)
			.add_systems(FixedUpdate, (move_entities, move_aliens, check_collisions))
			.add_systems(Update, (handle_player_input, animate))
		;
	}
}

fn create_world(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {	
	let asset_store = AssetStore::new(&asset_server, &mut texture_atlas_layouts);
	commands.spawn(create_camera_bundle());
	commands.insert_resource(AlienCollectiveState::new());
	commands.spawn(create_player_bundle(asset_store.player_texture.clone(), asset_store.player_atlas.clone()));
	commands.spawn_batch(create_aliens(asset_store.alien_textures.clone(), asset_store.alien_atlas.clone()));
	commands.insert_resource(asset_store);
}