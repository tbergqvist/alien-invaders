use bevy::{app::{App, PluginGroup}, render::{settings::{Backends, RenderCreation, WgpuSettings}, texture::ImagePlugin, RenderPlugin}, utils::default, window::{Window, WindowPlugin, WindowResolution}, DefaultPlugins};
use game_plugin::GamePlugin;

mod movement;
mod animation;
mod player;
mod alien;
mod collision;
mod bundles;
mod components;
mod game_plugin;

fn main() {
	App::new()
	.add_plugins(DefaultPlugins.set(RenderPlugin {
		render_creation: RenderCreation::Automatic(WgpuSettings {
			backends:Some(Backends::VULKAN),
			..default()
		}),
		..default()
	}).set(WindowPlugin {
		primary_window: Some(
			Window{
				//mode: bevy::window::WindowMode::BorderlessFullscreen,
				resolution: WindowResolution::new(1200., 1000.),
				..default()
			}
		),
		..default()
		})
		.set(ImagePlugin::default_nearest())
	)
	.add_plugins(GamePlugin)
	.run();
}
