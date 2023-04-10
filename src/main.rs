// Disable Windows console on release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::*, window::WindowResolution};
use bevy_kira_audio::AudioPlugin;
use bevy_pkv::PkvStore;

use intro::IntroPlugin;
#[cfg(debug_assertions)]
use debug::DebugPlugin;
use game::GamePlugin;
use menu::MenuPlugin;
use audio::{music::{MusicPlugin, Volume}, sfx::SFXPlugin};
use vfx::{post_processing::VFXPlugin, vfx_triggers::VFXTriggerPlugin};

use setup::*;

use game::states::{DayState, GameState, PauseState};

// Modules
mod intro;
mod game;
mod menu;
mod audio;
mod setup;
#[cfg(debug_assertions)]
mod debug;
mod vfx;

// Constants
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn main() {
	let height = 900.0;
	let mut app = App::new();
	app
		.add_plugins(DefaultPlugins
			.set(WindowPlugin {
				primary_window: Some(Window {
					// Fits screen to window size in wasm build
					//fit_canvas_to_parent: true,
					// Stops the game from stopping keyboard shortcuts e.g. F12
					prevent_default_event_handling: false,
					resolution: WindowResolution::new(height * ASPECT_RATIO, height).with_scale_factor_override(1.0),
					title: "A High Stakes Affair".to_string(),
					// Corresponds to VSync, framerate capped at refresh rate
					present_mode: bevy::window::PresentMode::Fifo,
					resizable: false,
					..default()
				}),
				..default()
			})
			.set(ImagePlugin::default_nearest())
		)
		.add_plugin(VFXPlugin::default())
		.add_plugin(VFXTriggerPlugin)
		.add_plugin(AudioPlugin)
		.add_state::<GameState>()
		.add_state::<DayState>()
		.add_state::<PauseState>()
		.add_plugin(GamePlugin)
		.add_plugin(MenuPlugin)
		.add_plugin(MusicPlugin)
		.add_plugin(SFXPlugin)
		.add_plugin(IntroPlugin)
		.insert_resource(PkvStore::new("InfiniteFallGames", "AHighStakesAffair"))
		.insert_resource(Volume(1.0))
		.insert_resource(UserData {
			name: "".to_string(),
			money: 0.0, stress: 0.0, flirt: 0.0, royal: 0.0,
			unstable_royal: 0.0, drugs_taken: 0.0,
			day: 0.0,	time: DayState::Dawn, lvl_init: false,
		})
		// To ensure the image handle resource gets spawned before spawn_camera
		.add_startup_systems((apply_system_buffers, spawn_camera).chain().after(vfx::post_processing::vfx_setup))
		.add_startup_system(spawn_cursor)
		.add_system(track_cursor);
	{
		#[cfg(debug_assertions)]
		app.add_plugin(DebugPlugin);
	}
	app.run();
}
