use bevy::{prelude::{Query, Res}, sprite::TextureAtlas, time::Time};

use crate::components::Animated;

pub fn animate(
	time: Res<Time>,
	mut query: Query<(&mut Animated, &mut TextureAtlas)>,
) {
	for (mut animated, mut texture_atlas) in &mut query {
		if animated.timer.tick(time.delta()).just_finished() {
			texture_atlas.index = if texture_atlas.index >= animated.end {
				animated.start
			} else {
				texture_atlas.index + 1
			};
		}
	}
}