use bevy::prelude::*;

use crate::game::states::DayState;

use super::{physics::Gravity, pegs::{Peg, PegType}, balls::Ball, food::DropType};

pub struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_system(adjust_gravity.run_if(not(in_state(DayState::Dawn))))
		.add_system(eyeball)
		;
    }
}

fn adjust_gravity(
	mut gravity: ResMut<Gravity>,
	keyboard: Res<Input<KeyCode>>,
) {
	if keyboard.just_pressed(KeyCode::I) {
		gravity.0 = (gravity.0 - 100.0).clamp(-1000.0, 0.0);
		println!("Gravity = {}", gravity.0);
	}
	if keyboard.just_pressed(KeyCode::O) {
		gravity.0 = (gravity.0 + 100.0).clamp(-1000.0, 0.0);
		println!("Gravity = {}", gravity.0);
	}
}


fn eyeball(
	asset_server: Res<AssetServer>,
	mut peg_query: Query<(&mut bevy::asset::Handle<Image>, &Peg, With<Peg>)>,
	mut ball_query: Query<(&mut bevy::asset::Handle<Image>, &DropType, (With<Ball>, Without<Peg>))>,
	keyboard: Res<Input<KeyCode>>,
) {
	if keyboard.just_pressed(KeyCode::G) {
		for (mut handle, _, _) in peg_query.iter_mut() {
			let new_handle: bevy::asset::Handle<Image> = asset_server.load("droppables/eyeball.png");
			*handle = new_handle.clone();
		}
		for (mut handle, _, _) in ball_query.iter_mut() {
			let new_handle: bevy::asset::Handle<Image> = asset_server.load("droppables/eyeball.png");
			*handle = new_handle.clone();
		}
	}

	if keyboard.just_pressed(KeyCode::H) {
		for (mut handle, peg, _) in peg_query.iter_mut() {
			let drop_path = match peg.0 {
				PegType::PachinkoPeg => "sprites/peg.png",
				PegType::ItemPeg(peg_type) => peg_type.get_path(),
			};
			let new_handle: bevy::asset::Handle<Image> = asset_server.load(drop_path);
			*handle = new_handle.clone();
		}
		for (mut handle, ball, _) in ball_query.iter_mut() {
			let new_handle: bevy::asset::Handle<Image> = asset_server.load(ball.get_path());
			*handle = new_handle.clone();
		}
	}
}