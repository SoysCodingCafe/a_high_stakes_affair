use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use rand::seq::SliceRandom;
use crate::{game::physics::BallCollisionEvent, setup::UserData};

use super::music::Volume;

pub struct SFXPlugin;

impl Plugin for SFXPlugin {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(DingTimer(Timer::from_seconds(0.2, TimerMode::Once)))
		.add_system(tick_timer)
		.add_system(ball_collision_sound.run_if(on_event::<BallCollisionEvent>()))
		;
	}
}

#[derive(Resource)]
pub struct DingTimer(pub Timer);

fn tick_timer(
	mut ding_timer: ResMut<DingTimer>,
	time: Res<Time>,
) {
	ding_timer.0.tick(time.delta());
}

fn ball_collision_sound(
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut ding_timer: ResMut<DingTimer>,
	user_data: Res<UserData>,
	volume: Res<Volume>,
) {
	if ding_timer.0.finished() {
		let mut sounds = vec![
			"audio/sfx/pegging.ogg",
			"audio/sfx/peg1.ogg",
			"audio/sfx/peg2.ogg",
			"audio/sfx/peg3.ogg",
			"audio/sfx/peg4.ogg",
		];
		if user_data.royal > 50.0 {
			let mut sounds_st = vec![
				"audio/sfx/peg_st1.ogg",
				"audio/sfx/peg_st2.ogg",
				"audio/sfx/peg_st3.ogg",
				"audio/sfx/peg_st4.ogg",
			];

			sounds.append(&mut sounds_st);
		}

		// Old method of randomly selecting sfx
		//let rn = ((rand::random::<f32>() * 3.0).floor() as usize).clamp(0, 2);
		//let sfx = asset_server.load(sounds[rn]);

		let sfx = asset_server.load(*sounds.choose(&mut rand::thread_rng()).unwrap());
		audio.play(sfx).with_volume(volume.0);

		ding_timer.0.reset();
	}
}
