use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{game::states::{GameState, DayState}, setup::UserData};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_system(audio_volume_control)
		.add_system(play_menu_music.in_schedule(OnEnter(GameState::Menu)))
		.add_system(stop_music.in_schedule(OnExit(GameState::Menu)))
		.add_system(play_morning_music.in_schedule(OnEnter(DayState::Morning)))
		.add_system(stop_music.in_schedule(OnExit(DayState::Morning)))
		.add_system(play_evening_music.in_schedule(OnEnter(DayState::Evening)))
		.add_system(stop_music.in_schedule(OnExit(DayState::Evening)))
		.add_system(play_night_music.in_schedule(OnEnter(DayState::Night)))
		.add_system(stop_music.in_schedule(OnExit(DayState::Night)))
		;
	}
}

#[derive(Resource)]
pub struct Volume(pub f64);


// Create generic "play music" function which checks next scene during transistion and loads
// the correct audio file
pub fn play_menu_music(
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
) {
	audio.play(asset_server.load("audio/bgm/menu_music.ogg"))
	.fade_in(AudioTween::linear(Duration::new(2, 0)))
	.looped();
}

pub fn play_morning_music(
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	user_data: Res<UserData>,
) {
	if user_data.royal > 50.0 {
		audio.play(asset_server.load("audio/bgm/deep_fried_sunset.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	} else {
		audio.play(asset_server.load("audio/bgm/jpeg_of_a_sunset.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	}
}

pub fn play_evening_music(
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	user_data: Res<UserData>,
) {
	if user_data.royal > 50.0 {
		audio.play(asset_server.load("audio/bgm/Mars_Attaque.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	} else {
		audio.play(asset_server.load("audio/bgm/Le_Pique_Nique_Martien.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	}
}

pub fn play_night_music(
	audio: Res<Audio>,
	asset_server: Res<AssetServer>,
	user_data: Res<UserData>,
) {
	if user_data.royal > 50.0 {
		audio.play(asset_server.load("audio/bgm/parlour_sugar_rush.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	} else {
		audio.play(asset_server.load("audio/bgm/parlour_sober_final.ogg"))
		.fade_in(AudioTween::linear(Duration::new(2, 0)))
		.looped();
	}
}

pub fn stop_music(
	audio: Res<Audio>,
) {
	audio.stop().fade_out(AudioTween::linear(Duration::new(2,0)));
}

pub fn audio_volume_control(
	keyboard: Res<Input<KeyCode>>,
	audio: Res<Audio>,
	mut volume: ResMut<Volume>,
) {
	if keyboard.just_pressed(KeyCode::Key0) {
		audio.set_volume(0.1);
		volume.0 = 0.0;
	}
	if keyboard.just_pressed(KeyCode::Key1) {
		audio.set_volume(0.2);
		volume.0 = 0.1;
	}
	if keyboard.just_pressed(KeyCode::Key2) {
		audio.set_volume(0.3);
		volume.0 = 0.2;
	}
	if keyboard.just_pressed(KeyCode::Key3) {
		audio.set_volume(0.4);
		volume.0 = 0.3;
	}
	if keyboard.just_pressed(KeyCode::Key4) {
		audio.set_volume(0.5);
		volume.0 = 0.4;
	}
	if keyboard.just_pressed(KeyCode::Key5) {
		audio.set_volume(0.6);
		volume.0 = 0.5;
	}
	if keyboard.just_pressed(KeyCode::Key6) {
		audio.set_volume(0.7);
		volume.0 = 0.6;
	}
	if keyboard.just_pressed(KeyCode::Key7) {
		audio.set_volume(0.8);
		volume.0 = 0.7;
	}
	if keyboard.just_pressed(KeyCode::Key8) {
		audio.set_volume(0.9);
		volume.0 = 0.8;
	}
	if keyboard.just_pressed(KeyCode::Key9) {
		audio.set_volume(1.0);
		volume.0 = 0.9;
	}
	if keyboard.just_pressed(KeyCode::M) {
		audio.set_volume(0.0);
		volume.0 = 1.0;
	}
}