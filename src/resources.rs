use bevy::{asset::{AssetServer, Assets, Handle}, math::{Vec2, Vec3}, prelude::{Component, Resource}, render::texture::Image, sprite::TextureAtlasLayout, time::{Timer, TimerMode}, transform::components::{GlobalTransform, Transform}};

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

#[derive(Component)]
pub struct AlienCollective {
	pub moving_direction: f32,
	pub shoot_timer: Timer,
}

impl AlienCollective {
	pub fn new() -> Self {
		Self {
			moving_direction: 1.,
			shoot_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
		}
	}
}