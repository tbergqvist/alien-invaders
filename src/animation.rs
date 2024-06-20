use bevy::{prelude::{Query, Res}, sprite::TextureAtlas, time::Time};

use crate::components::{AnimationIndexes, AnimationTimer};

pub fn animate(
	time: Res<Time>,
	mut query: Query<(&AnimationIndexes, &mut AnimationTimer, &mut TextureAtlas)>,
) {
	for (indexes, mut timer, mut texture_atlas) in &mut query {
		if timer.0.tick(time.delta()).just_finished() {
			texture_atlas.index = if texture_atlas.index >= indexes.end {
				indexes.start
			} else {
				texture_atlas.index + 1
			};
		}
	}
}