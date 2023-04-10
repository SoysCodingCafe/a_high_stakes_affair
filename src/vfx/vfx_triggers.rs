use bevy::prelude::*;

use crate::{
	game::{physics::{BallCollisionEvent, BallCollisionEventType}, pegs::PegType, food::{DropType, DropCategory}, states::{DayState, GameState}}, 
	setup::UserData
};
use super::PostProcessingSettings;

pub struct VFXTriggerPlugin;

impl Plugin for VFXTriggerPlugin {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(VFXTriggerSettings {following_velocity: 0.1, unstable_royal_decay: 1.0, ripple_probability: 0.1})
		//.add_system(ripple_ball_collision.in_set(super::VFXChangeSystemSet).run_if(on_event::<BallCollisionEvent>()))
		.add_system(ripple_mouth_collision.in_set(super::VFXChangeSystemSet).run_if(on_event::<BallCollisionEvent>()))
		.add_system(drug_implosion.run_if(on_event::<BallCollisionEvent>()))
		.add_system(follow_stats.in_set(super::VFXChangeSystemSet))
		;
	}
}

#[derive(Resource, Default, Clone, Debug)]
pub struct VFXTriggerSettings {
	pub following_velocity: f32,
	pub unstable_royal_decay: f32,
	pub ripple_probability: f32
}

/*fn ripple_ball_collision(
	mut events: EventReader<BallCollisionEvent>,
	mut post_processing_settings: ResMut<PostProcessingSettings>,
	windows: Query<&Window>,
	time: Res<Time>,
	trigger_settings: Res<VFXTriggerSettings>,
) {
	let window = windows.get_single().expect("Single window expected!");
	// How far has the previous ripple travelled in UV coords
	let dist = (time.elapsed_seconds() - post_processing_settings.ripple.start_time) * post_processing_settings.ripple.velocity;
	if trigger_settings.ripple_probability > 0.0001 && dist > 2.0 {
		for e in events.iter() {
			if let BallCollisionEventType::Peg {..} = e.collision_type {
				if rand::random::<f32>() < trigger_settings.ripple_probability {
					post_processing_settings.ripple.start_time = time.elapsed_seconds();
					post_processing_settings.ripple.start_x = e.pos.x / window.physical_width() as f32;
					post_processing_settings.ripple.start_y = 1.0 - e.pos.y / window.physical_height() as f32;
				}
			}
		}
	} else {
		events.clear();
	}
}*/

fn ripple_mouth_collision(
	mut events: EventReader<BallCollisionEvent>,
	mut post_processing_settings: ResMut<PostProcessingSettings>,
	windows: Query<&Window>,
	time: Res<Time>,
	current_state: Res<State<DayState>>,
) {
	let window = windows.get_single().expect("Single window expected!");
	// How far has the previous ripple travelled in UV coords
	let dist = (time.elapsed_seconds() - post_processing_settings.ripple.start_time) * post_processing_settings.ripple.velocity;
	if dist > 1.0 {
		for e in events.iter() {
			if let BallCollisionEventType::Peg {peg_type: PegType::ItemPeg(drop_type), ball_type, ..} = e.collision_type {
				if (!drop_type.is_edible() && ball_type == Some(DropType::Mouth)) 
				|| (current_state.0 == DayState::Night && drop_type == DropType::Bell && ball_type == Some(DropType::Ball)) {
					post_processing_settings.ripple.start_time = time.elapsed_seconds();
					post_processing_settings.ripple.start_x = e.pos.x / window.physical_width() as f32;
					post_processing_settings.ripple.start_y = 1.0 - e.pos.y / window.physical_height() as f32;
				}
			}
		}
	} else {
		events.clear();
	}
}

fn drug_implosion(
	_trigger_settings: Res<VFXTriggerSettings>,
	mut events: EventReader<BallCollisionEvent>,
	mut user_data: ResMut<UserData>,
) {
	let mut trigger_implosion = false;
	for &e in events.iter() {
		match e.collision_type {
			BallCollisionEventType::Peg { ball_type: Some(DropType::Mouth), peg_type: PegType::ItemPeg(item), .. } => {
				if item.is_edible() && item.get_type() == DropCategory::Drug {
					trigger_implosion = true;
					break;
				}
			}
			_ => {}
		}
	}
	if trigger_implosion {
		// Makes screen go crazy when you get a hit, quickly diminishing returns tho
		let effect_strength = 200.0;
		//println!("Royal: {}", user_data.royal);
		//println!("Unstable Royal: {}", user_data.royal);
		user_data.unstable_royal = f32::max(user_data.unstable_royal,
			1.0 / (1.0 / effect_strength + f32::max(0.0, user_data.royal as f32 / 100.0)));
	}
}

fn follow_stats(
	mut settings: ResMut<PostProcessingSettings>,
	trigger_settings: Res<VFXTriggerSettings>,
	mut user_data: ResMut<UserData>,
	time: Res<Time>,
	game_state: Res<State<GameState>>,
) {

	if game_state.0 == GameState::Boot {
		settings.strength.contrast_distort = 0.0;
		settings.strength.hue_shift = 0.0;
		settings.strength.wave_distort = 0.0;
		return;
	}

	let norm_royal = user_data.royal as f32 / 100.0 + user_data.unstable_royal;
	let desired_wave = norm_royal * 0.2;
	let desired_hues = norm_royal * 5.0;
	let desired_contrast = user_data.stress as f32 / 250.0;
	// This makes it follow an exponential curve towards desired value
	let rel_delta = time.delta_seconds() * trigger_settings.following_velocity;
	settings.strength.contrast_distort += rel_delta * (desired_contrast - settings.strength.contrast_distort);
	settings.strength.wave_distort += rel_delta * (desired_wave - settings.strength.wave_distort);
	settings.strength.hue_shift	   += rel_delta * (desired_hues - settings.strength.hue_shift	);
	// unstable royal decays slowly
	user_data.unstable_royal -= time.delta_seconds() * trigger_settings.unstable_royal_decay * user_data.unstable_royal.signum();
	if user_data.unstable_royal < 0.0 {
		user_data.unstable_royal = 0.0;
	}
	settings.strength.contrast_distort = settings.strength.contrast_distort.clamp(-1.0, 1.0);
	settings.strength.wave_distort = settings.strength.wave_distort.clamp(0.0, 1.6);
	settings.strength.hue_shift = settings.strength.hue_shift.clamp(0.0, 5.0);
}
