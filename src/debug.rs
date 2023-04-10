use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::game::{states::{GameState, DayState, PauseState}, pegs::Peg, targets::ClearOnDayTransition};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
		if cfg!(debug_assertions) {
        app
		.add_system(manually_change_post_processing_settings.in_set(crate::vfx::VFXChangeSystemSet))
		.add_plugin(WorldInspectorPlugin::new())
		.add_systems((
			place_peg
				.run_if(in_state(GameState::Game))
				.run_if(not(in_state(DayState::Dawn))),
			print_map
				.run_if(in_state(GameState::Game))
				.run_if(not(in_state(DayState::Dawn))),
			set_day_state
				.run_if(in_state(GameState::Game)),
			set_game_state
		))
		;
		}
	}
}

pub fn manually_change_post_processing_settings(
	keyboard: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut settings: ResMut<crate::vfx::PostProcessingSettings>,
) {
	if keyboard.just_pressed(KeyCode::Numpad0) {
		settings.strength.wave_distort = 0.0;
	}
	if keyboard.just_pressed(KeyCode::Numpad1) {
		settings.strength.wave_distort += 0.15;
	}
	if keyboard.just_pressed(KeyCode::Numpad2) {
		settings.strength.wave_distort -= 0.05;
	}
	if keyboard.just_pressed(KeyCode::Numpad3) {
		settings.ripple.start_time = time.elapsed_seconds();
	}
	if keyboard.just_pressed(KeyCode::Numpad4) {
		settings.strength.hue_shift += 0.1;
	}
	if keyboard.just_pressed(KeyCode::Numpad5) {
		settings.strength.hue_shift -= 0.1;
	}
	if keyboard.just_pressed(KeyCode::Numpad7) {
		settings.strength.contrast_distort += 0.05;
		info!("Contrast boost: {}", settings.strength.contrast_distort);
	}
	if keyboard.just_pressed(KeyCode::Numpad8) {
		settings.strength.contrast_distort -= 0.05;
		info!("Contrast boost: {}", settings.strength.contrast_distort);
	}
	//settings.strength.contrast_distort = settings.strength.contrast_distort.clamp(-1.0, 1.0);
	//settings.strength.wave_distort = settings.strength.wave_distort.clamp(0.0, 1.0);
	//settings.strength.hue_shift = settings.strength.hue_shift.clamp(0.0, 5.0);
}

pub fn set_day_state(
	keyboard: Res<Input<KeyCode>>,
	state: Res<State<DayState>>,
	mut next_state: ResMut<NextState<DayState>>,
) {
	if keyboard.just_pressed(KeyCode::Q) {
		if state.0 != DayState::Morning {
			next_state.set(DayState::Morning);
			println!("Day State Morning");
		}
	}
	if keyboard.just_pressed(KeyCode::W) {
		if state.0 != DayState::Evening {
			next_state.set(DayState::Evening);
			println!("Day State Evening");
		}
	}
	if keyboard.just_pressed(KeyCode::E) {
		if state.0 != DayState::Night {
			next_state.set(DayState::Night);
			println!("Day State Night");
		}
	}
}

// Level Editor Plugin
#[derive(Component)]
struct Placed;

fn place_peg(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
) {
	let window = window_query.get_single().unwrap();

	if mouse.just_pressed(MouseButton::Right) {
		if let Some(current_pos) = window.cursor_position() {
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(current_pos.x, current_pos.y, 0.0),
						texture: asset_server.load("sprites/peg.png"),
						..default()
					},
					Peg::default(),
					Placed,
					ClearOnDayTransition,
				)
			);
		}
	}
}

fn print_map(
	peg_query: Query<(&Peg, &Transform), With<Placed>>,
	key_press: Res<Input<KeyCode>>,
) {
	if key_press.just_pressed(KeyCode::M) {
		print!("let map = vec![");
		for (_, transform) in peg_query.iter() {
			print!("({:.1},{:.1}),", transform.translation.x, transform.translation.y)
		}
		println!("];");
	}
}

pub fn set_game_state(
	keyboard: Res<Input<KeyCode>>,
	pause_state: Res<State<PauseState>>,
	game_state: Res<State<GameState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut next_day_state: ResMut<NextState<DayState>>,
) {
	if keyboard.just_pressed(KeyCode::Z) {
		if game_state.0 != GameState::Menu {
			next_game_state.set(GameState::Menu);
			next_day_state.set(DayState::Dawn);
			println!("Game State Menu");
			if pause_state.0 != PauseState::Initial {
				next_pause_state.set(PauseState::Initial)
			}
		}
	}
	if keyboard.just_pressed(KeyCode::X) {
		if game_state.0 != GameState::Intro {
			next_game_state.set(GameState::Intro);
			next_day_state.set(DayState::Dawn);
			println!("Game State Intro");
			if pause_state.0 != PauseState::Initial {
				next_pause_state.set(PauseState::Initial)
			}
		}
	}
	if keyboard.just_pressed(KeyCode::C) {
		if game_state.0 != GameState::Game {
			next_game_state.set(GameState::Game);
			next_day_state.set(DayState::Dawn);
			if pause_state.0 != PauseState::Initial {
				next_pause_state.set(PauseState::Initial)
			}
			println!("Game State Game");
		}
	}
	if keyboard.just_pressed(KeyCode::L) {
		if game_state.0 != GameState::Outro {
			next_game_state.set(GameState::Outro);
			next_day_state.set(DayState::Dawn);
			println!("Game State GameOver");
			if pause_state.0 != PauseState::Initial {
				next_pause_state.set(PauseState::Initial)
			}
		}
	}
}