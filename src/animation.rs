use bevy::{prelude::{Component, Query, Res}, sprite::TextureAtlas, time::{Time, Timer, TimerMode}};

#[derive(Component)]
pub struct AnimationIndexes {
	pub start: usize,
	pub end: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub fn animate(
	time: Res<Time>,
	mut query: Query<(&AnimationIndexes, &mut AnimationTimer, &mut TextureAtlas)>,
) {
	for (indexes, mut timer, mut texture_atlas) in &mut query {
		if timer.0.tick(time.delta()).just_finished() {
			texture_atlas.index = if texture_atlas.index == indexes.end {
				indexes.start
			} else {
				texture_atlas.index + 1
			};

			timer.0 = Timer::from_seconds(1., TimerMode::Once);
		}
	}
}