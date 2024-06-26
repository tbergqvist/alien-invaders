use bevy::{asset::{AssetServer, Assets, Handle}, math::Vec2, prelude::{Resource, States}, render::texture::Image, sprite::TextureAtlasLayout, time::{Timer, TimerMode}};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
	Loading,
	InGame,
}

#[derive(Resource)]
pub struct AssetStore {
	pub player_texture: Handle<Image>,
	pub shield_base: Handle<Image>,
	pub shield_textures: Vec<Handle<Image>>,
	pub alien_textures: Vec<Handle<Image>>,
	pub player_projectile_texture: Handle<Image>,
	pub player_atlas: Handle<TextureAtlasLayout>,
	pub alien_atlas: Handle<TextureAtlasLayout>,
	pub projectile_atlas: Handle<TextureAtlasLayout>,
	pub explosion_texture: Handle<Image>,
	pub explosion_atlas: Handle<TextureAtlasLayout>,
}

impl AssetStore {
	pub fn new(
		asset_server: &AssetServer,
		texture_atlas_layouts: &mut Assets<TextureAtlasLayout>
	) -> Self {
		let player_texture = asset_server.load("player.png");
		let shield_base = asset_server.load("shield.png");
		let alien_textures = vec![asset_server.load("alien1.png"), asset_server.load("alien2.png"), asset_server.load("alien3.png")];
		let player_projectile_texture = asset_server.load("player_projectile.png");
		let explosion_texture = asset_server.load("explosion.png");
		let player_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 1, None, None));
		let alien_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None));
		let projectile_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(1.0, 8.0), 4, 1, None, None));
		let explosion_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None));

		Self {
			player_texture,
			alien_textures,
			shield_base,
			shield_textures: vec![],
			player_projectile_texture,
			player_atlas: player_layout,
			alien_atlas: alien_layout,
			projectile_atlas: projectile_layout,
			explosion_atlas,
			explosion_texture
		}
	}
}

#[derive(Resource)]
pub struct AlienCollectiveState {
	pub moving_direction: f32,
	pub shoot_timer: Timer,
}

impl AlienCollectiveState {
	pub fn new() -> Self {
		Self {
			moving_direction: 1.,
			shoot_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
		}
	}
}