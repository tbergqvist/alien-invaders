use bevy::{hierarchy::BuildChildren, prelude::{Commands, Component, Query, ResMut, With}, render::color::Color, text::{Text, TextStyle}, ui::{node_bundles::{NodeBundle, TextBundle}, PositionType, Style, UiRect, UiScale}, utils::default};

use crate::components::{Health, Player};

#[derive(Component)]
pub struct PlayerHealthText;

pub fn setup_gui(mut commands: Commands, mut ui_scale: ResMut<UiScale>) {
	ui_scale.0 = 4.;
	let health_text = (
		TextBundle::from_section(
			"HP".to_string(),
			TextStyle {
				font_size: 10.,
				color: Color::WHITE,
				..default()
			}
		).with_style(Style {
			position_type: PositionType::Relative,
			margin: UiRect::px(0., 4., 8., 0.),
			..Default::default()
		}),
		PlayerHealthText
	);

	commands.spawn(NodeBundle {
		style: Style {
			position_type: PositionType::Absolute,
			bottom: bevy::prelude::Val::Px(20.),
			left: bevy::prelude::Val::Px(0.),
			display: bevy::ui::Display::Flex,
			justify_content: bevy::ui::JustifyContent::Center,
			align_items: bevy::ui::AlignItems::Center,
			height: bevy::prelude::Val::Px(20.),
			..Default::default()
		},
		..default()
	})
	.with_children(|parent| {
		parent.spawn(health_text);
	});
}

pub fn update_player_counter(mut text_query: Query<&mut Text, With<PlayerHealthText>>, player_query: Query<&Health, With<Player>>) {
	let mut hp_text= text_query.single_mut();
	let player = player_query.get_single();
	if let Ok(health) = player {
		hp_text.sections[0].value = format!("HP:{}", health.hp);
	} else {
		hp_text.sections[0].value = "HP:0".to_string();
	}
}