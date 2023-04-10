use bevy::{prelude::*, asset::LoadState};

use crate::game::states::GameState;

pub struct WarningPlugin;

impl Plugin for WarningPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_system(show_warning.in_schedule(OnEnter(GameState::Boot)))
		.add_system(handle_close_button.run_if(in_state(GameState::Boot)))
		;
	}
}

#[derive(Component)]
struct WarningDisclaimer;

fn show_warning(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				texture: asset_server.load("intro/warning.png"),
				transform: Transform::from_xyz(800.0, 450.0, 0.0),
				..default()
			},
			WarningDisclaimer,
		)
	);
}

fn handle_close_button(
	mut commands: Commands,
	disclaimer: Query<(Entity, &Handle<Image>), With<WarningDisclaimer>>,
	asset_server: Res<AssetServer>,
	mouse: Res<Input<MouseButton>>,
	mut next_game_state: ResMut<NextState<GameState>>,
) {
	let mut loaded = true;
	for (_, image) in disclaimer.iter() {
		if asset_server.get_load_state(image) != LoadState::Loaded {
			loaded = false;
		}
	}
	if loaded && mouse.just_released(MouseButton::Left) {
		next_game_state.set(GameState::Menu);
		for (e, _) in disclaimer.iter() {
			commands.entity(e).despawn_recursive();
		}
	}
}

/*
	next_day_state.set(DayState::Dawn);
	next_pause_state.set(PauseState::Paused);
		
	mut next_day_state: ResMut<NextState<DayState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
		 */