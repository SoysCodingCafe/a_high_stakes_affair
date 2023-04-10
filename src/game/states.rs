use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Serialize, Deserialize};

use crate::setup::{despawn_entities_with, UserData, SaveData};

use super::{PachinkoSystemSet, targets::ClearOnDayTransition, minigames::parlor::PrizeLaunchTimer};

pub const DAY_LENGTH: f32 = 75.0;

pub const FRAME_BACK_DEPTH: f32 = -1.0;
pub const FRAME_INNER_DEPTH: f32 = -0.5;
pub const FRAME_FRONT_DEPTH: f32 = -0.1;
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
	fn build(&self, app: &mut App) {
		let initialization_systems = (
			initialise_clock,
			initialise_wallet,
			initialise_stress,
			initialise_royal,
		);
		app
			.add_system(toggle_pause_state.run_if(in_state(GameState::Game)))

			.add_systems(initialization_systems.in_schedule(OnEnter(DayState::Morning)))
			.add_systems(initialization_systems.in_schedule(OnEnter(DayState::Evening)))
			.add_systems(initialization_systems.in_schedule(OnEnter(DayState::Night)))

			.add_system(initialise_money_timer.in_schedule(OnEnter(DayState::Night)))

			.add_systems((update_time, update_wallet, update_stress, update_royal).in_set(PachinkoSystemSet)
			.after(initialise_clock).after(initialise_wallet).after(initialise_stress).after(initialise_royal))
			.add_system(advance_time.in_set(PachinkoSystemSet).after(initialise_clock))

			.add_system(despawn_entities_with::<ClearOnDayTransition>.in_schedule(OnExit(GameState::Game)))
			.add_system(despawn_entities_with::<ClearOnDayTransition>.in_schedule(OnExit(DayState::Morning)))
			.add_system(despawn_entities_with::<ClearOnDayTransition>.in_schedule(OnExit(DayState::Evening)))
			.add_system(despawn_entities_with::<ClearOnDayTransition>.in_schedule(OnExit(DayState::Night)))
			;
	}
}

// Game State
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
	#[default]
	Boot,
	Menu,
	Intro,
	Game,
	Outro,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub enum DayState {
	#[default]
	Dawn,
	Morning,
	Evening,
	Night,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PauseState {
	#[default]
	Initial,
	Paused,
	Unpaused,
}

#[derive(Resource)]
pub struct DayTimer(pub Timer);

#[derive(Resource)]
pub struct OutOfMoneyTimer(pub Timer);

#[derive(Component)]
pub struct Clock;

#[derive(Component)]
pub struct Wallet;

#[derive(Component)]
pub struct Stress;

#[derive(Component)]
pub struct Royal;

#[derive(Component)]
pub struct Frame;

pub fn initialise_money_timer(
	mut commands: Commands,
) {
	commands.insert_resource(OutOfMoneyTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

pub fn initialise_clock(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(DayTimer(Timer::from_seconds(DAY_LENGTH, TimerMode::Once)));
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(40.0, 832.0, FRAME_INNER_DEPTH)
					.with_scale(Vec3::new(0.0, 1.0, 1.0)),
				texture: asset_server.load("sprites/clock.png"),
				..default()
			},
			Clock,
			ClearOnDayTransition
		)
	);
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(140.0, 832.0, FRAME_FRONT_DEPTH),
				texture: asset_server.load("sprites/frame_front.png"),
				..default()
			},
			Frame,
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(140.0, 832.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/frame_back.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(140.0, 878.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/time.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);
}

pub fn initialise_wallet(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(280.0, 832.0, FRAME_INNER_DEPTH)
					.with_scale(Vec3::new(0.0, 1.0, 1.0)),
				texture: asset_server.load("sprites/wallet.png"),
				..default()
			},
			Wallet,
			ClearOnDayTransition
		)
	);
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(380.0, 832.0, FRAME_FRONT_DEPTH),
				texture: asset_server.load("sprites/frame_front.png"),
				..default()
			},
			ClearOnDayTransition,
			Frame,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(380.0, 832.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/frame_back.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(380.0, 878.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/money.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);
}

pub fn initialise_stress(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(520.0, 832.0, FRAME_INNER_DEPTH)
					.with_scale(Vec3::new(0.0, 1.0, 1.0)),
				texture: asset_server.load("sprites/stress.png"),
				..default()
			},
			Stress,
			ClearOnDayTransition
		)
	);
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(620.0, 832.0, FRAME_FRONT_DEPTH),
				texture: asset_server.load("sprites/frame_front.png"),
				..default()
			},
			Frame,
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(620.0, 832.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/frame_back.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(620.0, 878.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/stresss.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);
}

pub fn initialise_royal(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(760.0, 832.0, FRAME_INNER_DEPTH)
					.with_scale(Vec3::new(0.0, 1.0, 1.0)),
				texture: asset_server.load("sprites/royal.png"),
				..default()
			},
			Royal,
			ClearOnDayTransition
		)
	);
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(860.0, 832.0, FRAME_FRONT_DEPTH),
				texture: asset_server.load("sprites/frame_front.png"),
				..default()
			},
			Frame,
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(860.0, 832.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/frame_back.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);

	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(860.0, 878.0, FRAME_BACK_DEPTH),
				texture: asset_server.load("sprites/highness.png"),
				..default()
			},
			ClearOnDayTransition,
		)
	);
}

pub fn update_time(
	day_timer: Res<DayTimer>,
	mut clock_query: Query<&mut Transform, With<Clock>>,
) {
	if let Ok(mut transform) = clock_query.get_single_mut() {
		transform.scale.x = day_timer.0.percent() * 200.0 / 32.0;
		transform.translation.x = (day_timer.0.percent() * 200.0 / 2.0) + 40.0;
	}
}

pub fn update_wallet(
	mut wallet_query: Query<&mut Transform, With<Wallet>>,
	user_data: Res<UserData>,
) {
	if let Ok(mut transform) = wallet_query.get_single_mut() {
		transform.scale.x = user_data.money as f32 / 100.0 * 200.0 / 32.0;
		transform.translation.x = user_data.money as f32 / 100.0 * 200.0 / 2.0 + 280.0;
	}
	}

pub fn update_stress(
	mut stress_query: Query<&mut Transform, With<Stress>>,
	user_data: Res<UserData>,
) {
	if let Ok(mut transform) = stress_query.get_single_mut() {
		transform.scale.x = user_data.stress as f32 / 100.0 * 200.0 / 32.0;
		transform.translation.x = user_data.stress as f32 / 100.0 * 200.0 / 2.0 + 520.0;
	}
}

pub fn update_royal(
	mut royal_query: Query<&mut Transform, With<Royal>>,
	user_data: Res<UserData>,
) {
	if let Ok(mut transform) = royal_query.get_single_mut() {
		transform.scale.x = user_data.royal as f32 / 100.0 * 200.0 / 32.0;
		transform.translation.x = user_data.royal as f32 / 100.0 * 200.0 / 2.0 + 760.0;
	}
}

pub fn advance_time(
	current_day_state: Res<State<DayState>>,
	time: Res<Time>,
	mut day_timer: ResMut<DayTimer>,
	mut next_day_state: ResMut<NextState<DayState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut pkv: ResMut<PkvStore>,
	mut user_data: ResMut<UserData>,
	prize_launch_timer: Res<PrizeLaunchTimer>,
) {
	day_timer.0.tick(time.delta());
	if !(prize_launch_timer.0.percent() > 0.0) {
		if day_timer.0.finished() {
			let next = match current_day_state.0 {
				DayState::Dawn => DayState::Morning,
				DayState::Morning => DayState::Evening,
				DayState::Evening => DayState::Night,
				DayState::Night => DayState::Morning,
			};
			if let Ok(mut save_data) = pkv.get::<SaveData>("user_info") {
				// Save progress here
				save_data.money = user_data.money;
				save_data.stress = user_data.stress;
				save_data.flirt = user_data.flirt;
				save_data.royal = user_data.royal;
				save_data.drugs_taken = user_data.drugs_taken;

				if next == DayState::Morning {
					user_data.day = user_data.day + 1.0;
					save_data.day = user_data.day;
					if save_data.royal > 50.0 {
						save_data.royal -= 50.0;
						user_data.royal -= 50.0;
					} else if save_data.royal > 20.0 {
						save_data.royal -= 10.0;
						user_data.royal -= 10.0;
					}
				}
				user_data.time = next;
				save_data.time = next;
				user_data.lvl_init = false;
				save_data.lvl_init = false;
				pkv.set("user_info", &save_data)
					.expect("Unable to store user");
			}
			next_day_state.set(next);
			next_pause_state.set(PauseState::Paused);
		}
	}
}

pub fn toggle_pause_state(
	keyboard: Res<Input<KeyCode>>,
	state: Res<State<PauseState>>,
	mut next_state: ResMut<NextState<PauseState>>,
) {
	if keyboard.just_pressed(KeyCode::P) {
		if state.0 != PauseState::Unpaused {
			next_state.set(PauseState::Unpaused);
			println!("Pause State Unpaused");
		}
		else {
			next_state.set(PauseState::Paused);
			println!("Pause State Paused");
		}
	}
}
