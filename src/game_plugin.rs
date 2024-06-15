use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::{component::Component, query::With, system::{Commands, Query, Res, ResMut}}, input::{keyboard::KeyCode, ButtonInput}, math::{Vec2, Vec3}, prelude::Camera2dBundle, render::{camera::{ClearColor, ScalingMode}, color::Color}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, transform::components::Transform, utils::default, window::WindowResolution};

pub struct GamePlugin;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(f32);

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ClearColor(Color::BLACK))
			.add_systems(Startup, create_world)
			.add_systems(FixedUpdate, move_player)
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
	let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 1, 2, None, None);
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
}

fn handle_player_input(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
	let mut player_vel = query.single_mut();

	if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
		player_vel.0 = -1.;
	} else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
		player_vel.0 = 1.;
	} else {
		player_vel.0 = 0.;
	}
}

fn move_player(mut query: Query<(&mut Transform, &Velocity), With<Player>>) {
	for (mut transform, velocity) in query.iter_mut() {
		let new_pos = transform.translation.x + velocity.0;
		transform.translation.x = new_pos.clamp(0., 280.);
	}
}