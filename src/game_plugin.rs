use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::system::{Commands, Res, ResMut}, hierarchy::BuildChildren, prelude::{IntoSystemConfigs, SpatialBundle}, render::{camera::ClearColor, color::Color}, sprite::TextureAtlasLayout, transform::components::{GlobalTransform, Transform}, utils::default};

use crate::{alien::{alien_fire, create_aliens, move_aliens}, animation::animate, bundles::{create_camera_bundle, create_player_bundle}, collision::{check_collisions, decrease_health, remove_collided}, movement::move_entities, player::handle_player_input, resources::{AlienCollective, AssetStore}};

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
			.add_systems(Startup, create_world)
			.add_systems(FixedUpdate, (move_entities, move_aliens, alien_fire, collision_systems))
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
	commands.spawn(create_player_bundle(asset_store.player_texture.clone(), asset_store.player_atlas.clone()));

	commands.spawn((AlienCollective::new(), SpatialBundle::default())).with_children(|parent| {
		for alien in create_aliens(asset_store.alien_textures.clone(), asset_store.alien_atlas.clone()).into_iter() {
			parent.spawn(alien);
		}
	});
	
	commands.insert_resource(asset_store);
}