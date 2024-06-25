use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, prelude::IntoSystemConfigs, render::{camera::ClearColor, color::Color}, sprite::TextureAtlasLayout};

use crate::{alien::{alien_fire, create_aliens, move_aliens}, animation::animate, bundles::{create_camera_bundle, create_player_bundle}, collision::{check_collisions, decrease_health, kill_old_entities, remove_collided}, gui::{setup_gui, update_player_counter}, movement::move_entities, player::handle_player_input, resources::{AlienCollectiveState, AssetStore}};

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		let collision_systems = (
			check_collisions,
			decrease_health,
			remove_collided
		).chain();

		app
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, (create_world, setup_gui).chain())
			.add_systems(FixedUpdate, (move_entities, move_aliens, alien_fire, collision_systems, kill_old_entities))
			.add_systems(Update, (handle_player_input, animate, update_player_counter))
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