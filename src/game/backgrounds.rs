use bevy::prelude::*;

use crate::setup::{UserData, despawn_entities_with};

use crate::game::states::{DayState, GameState};

use super::targets::ClearOnDayTransition;

pub struct BackgroundsPlugin;

impl Plugin for BackgroundsPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_system(spawn_title.in_schedule(OnEnter(GameState::Menu)))
		.add_system(spawn_background.in_schedule(OnEnter(DayState::Morning)))
		.add_system(spawn_background.in_schedule(OnEnter(DayState::Evening)))
		.add_system(spawn_background.in_schedule(OnEnter(DayState::Night)))
		.add_system(despawn_entities_with::<Title>.in_schedule(OnExit(GameState::Menu)))
		;
	}
}

#[derive(Component)]
pub struct Title;

fn spawn_background(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	state: Res<State<DayState>>,
	user_data: Res<UserData>,
) {
	let day = user_data.day % 2.0;
	let background_sprite = match state.0 {
		DayState::Dawn => asset_server.load("backgrounds/mart.png"),
		DayState::Morning => asset_server.load("backgrounds/mart.png"),
		DayState::Evening => asset_server.load("backgrounds/cart.png"),
		DayState::Night => asset_server.load(format!("backgrounds/arcade{}.png", day)),
	};

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(800.0, 450.0, -10.0),
				texture: background_sprite,
				..default()
			},
			ClearOnDayTransition,
		)
	);
}

fn spawn_title(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(800.0, 450.0, -30.0),
				texture: asset_server.load("intro/title.png"),
				..default()
			},
			ClearOnDayTransition,
			Title,
		)
	);
}