use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::{component::Component, query::With, system::{Commands, Query, Res, ResMut}}, input::{keyboard::KeyCode, ButtonInput}, math::{Vec2, Vec3}, prelude::Camera2dBundle, render::{camera::{ClearColor, ScalingMode}, color::Color}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, transform::components::Transform, utils::default};

pub struct GamePlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Alien;

#[derive(Component)]
struct Velocity(f32);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, create_world)
			.add_systems(FixedUpdate, (move_entities))
			.add_systems(Update, handle_player_input)
		;
	}
}

fn create_world(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
	let player_texture = asset_server.load("player.png");
	let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 1, None, None);
	let texture_atlas_layout = texture_atlas_layouts.add(layout);
	let mut camera = Camera2dBundle::default();
	camera.transform = Transform::from_translation(Vec3::new(140., 100., 1.));
	camera.projection.scaling_mode = ScalingMode::WindowSize(4.);
	commands.spawn(camera);
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

	spawn_aliens(&mut commands, &asset_server, &mut texture_atlas_layouts);
}

fn handle_player_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
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

fn move_entities(mut query: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in query.iter_mut() {
		let new_pos = transform.translation.x + velocity.0;
		transform.translation.x = new_pos.clamp(0., 280.);
	}
}

fn spawn_aliens(commands: &mut Commands, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) {
	let alien1_spritesheet = get_spritesheet_component("alien1.png", asset_server, texture_atlas_layouts);
	let alien2_spritesheet = get_spritesheet_component("alien2.png", asset_server, texture_atlas_layouts);
	let alien3_spritesheet = get_spritesheet_component("alien3.png", asset_server, texture_atlas_layouts);

	spawn_specific_aliens(commands, 11, 200., alien1_spritesheet);
	spawn_specific_aliens(commands, 22, 180., alien2_spritesheet);
	spawn_specific_aliens(commands, 22, 140., alien3_spritesheet);
}

fn spawn_specific_aliens(commands: &mut Commands, amounts: i32, start_y: f32, spritesheet: SpriteSheetBundle) {
		for i in 0..amounts {
			let mut sheet = spritesheet.clone();
			let x = i % 11;
			let y = i / 11;
			sheet.transform.translation = Vec3::new(x as f32 * 20., start_y - (y as f32 * 20.), 0.);
			commands.spawn((sheet, Velocity(0.), Alien));
		}
}

fn get_spritesheet_component(path: &'static str, asset_server: &Res<AssetServer>, texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>) -> SpriteSheetBundle {
	let alien1_texture = asset_server.load(path);
	let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None);
	let texture_atlas_layout = texture_atlas_layouts.add(layout);
	SpriteSheetBundle {
		texture: alien1_texture,
		atlas: TextureAtlas {
			layout: texture_atlas_layout,
			index: 0,
		},
		..default()
	}
	}