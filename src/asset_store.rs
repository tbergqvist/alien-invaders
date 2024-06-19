use bevy::{asset::{AssetServer, Assets, Handle}, math::Vec2, prelude::Resource, render::texture::Image, sprite::TextureAtlasLayout};

#[derive(Resource)]
pub struct AssetStore {
		pub player_texture: Handle<Image>,
		pub alien_textures: Vec<Handle<Image>>,
		pub player_projectile_texture: Handle<Image>,
		pub player_atlas: Handle<TextureAtlasLayout>,
		pub alien_atlas: Handle<TextureAtlasLayout>,
		pub projectile_atlas: Handle<TextureAtlasLayout>,
}

impl AssetStore {
	pub fn new(
		asset_server: &AssetServer,
		texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
	) -> Self {
		let player_texture = asset_server.load("player.png");
		let alien_textures = vec![asset_server.load("alien1.png"), asset_server.load("alien2.png"), asset_server.load("alien3.png")];
		let player_projectile_texture = asset_server.load("player_projectile.png");
		let player_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 1, None, None));
		let alien_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None));
		let projectile_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(8.0, 8.0), 1, 2, None, None));
	
		Self {
			player_texture,
			alien_textures,
			player_projectile_texture,
			player_atlas: player_layout,
			alien_atlas: alien_layout,
			projectile_atlas: projectile_layout,
		}
	}
}