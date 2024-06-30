use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, prelude::{in_state, IntoSystemConfigs, Local, NextState, OnEnter}, render::{camera::ClearColor, color::Color, texture::Image}, sprite::TextureAtlasLayout};

use crate::{alien::{alien_fire, create_aliens, move_aliens}, animation::animate, bundles::{create_camera_bundle, create_player_bundle, create_shield_bundle}, collision::{check_collisions, decrease_health, kill_old_entities, remove_collided}, gui::{setup_gui, update_player_counter}, movement::move_entities, player::handle_player_input, resources::{AlienCollectiveState, AssetStore, GameState}};

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
			.add_systems(OnEnter(GameState::InGame), (create_world, setup_gui).chain())
			.add_systems(Startup, (load_resources).chain())
			.add_systems(FixedUpdate, (move_entities, move_aliens, alien_fire, collision_systems, kill_old_entities).run_if(in_state(GameState::InGame)))
			.add_systems(FixedUpdate, (check_assets_loading).run_if(in_state(GameState::Loading)))
			.add_systems(Update, (handle_player_input, animate, update_player_counter).run_if(in_state(GameState::InGame)))
			.insert_state(GameState::Loading)
		;
	}
}

fn create_world(
	mut commands: Commands,
	asset_store: Res<AssetStore>
) {	
	commands.spawn(create_camera_bundle());
	commands.insert_resource(AlienCollectiveState::new());
	commands.spawn(create_player_bundle(asset_store.player_texture.clone(), asset_store.player_atlas.clone()));
	
	for (i, texture) in asset_store.shield_textures.iter().enumerate() {
		commands.spawn(create_shield_bundle(texture, (i * 60 + 50) as f32));
	}

	commands.spawn_batch(create_aliens(asset_store.alien_textures.clone(), asset_store.alien_atlas.clone()));
}

fn load_resources(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
	let asset_store = AssetStore::new(&asset_server, &mut texture_atlas_layouts);
	commands.insert_resource(asset_store);
}

fn check_assets_loading(mut asset_store: ResMut<AssetStore>, mut images: ResMut<Assets<Image>>, mut state: ResMut<NextState<GameState>>, mut has_ran: Local<bool>) {
	if *has_ran {
		return;
	}

	if let Some(image) = images.get(&asset_store.shield_base) {
		let img = image.clone();
		asset_store.shield_textures = (0..4).into_iter().map(|_| {
			images.add(img.clone())
		})
		.collect();

		state.set(GameState::InGame);
		*has_ran = true;
	}
}