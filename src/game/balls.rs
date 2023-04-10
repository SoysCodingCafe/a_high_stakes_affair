// Made for general-purpose ball interactions
// Spawning and Despawning which is consistent across games

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_pkv::PkvStore;

use crate::despawn_entities_with;
use crate::setup::{UserData, SaveData};

use crate::game::states::{DayState, GameState, PauseState};

use super::PachinkoSystemSet;
use super::food::DropType;
use super::physics::Gravity;
use super::physics::Velocity;
use super::physics::ball_target_collide;
use super::states::OutOfMoneyTimer;

pub const BALL_DEPTH: f32 = -0.9;
pub const DROPZONE_DEPTH: f32 = -1.5;

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
	fn build(&self, app: &mut App) {
		let despawn_systems = (despawn_entities_with::<Ball>, despawn_entities_with::<DropSprite>);

		app
		.insert_resource(Gravity(-500.0))
		.insert_resource(
			DropZone{max_lim: Vec3::new(1000.0, 800.0, 0.0),
				min_lim: Vec3::new(0.0, 650.0, 0.0)})
		// Game systems
		.add_system(spawn_ball
			.run_if(not(in_state(DayState::Dawn)))
			.run_if(not(in_state(DayState::Morning)))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_systems((despawn_ball.after(ball_target_collide), launch_ball).in_set(PachinkoSystemSet))
		// Spawning systems
		.add_system(spawn_drop_zone.in_schedule(OnEnter(DayState::Morning)))
		.add_system(spawn_drop_zone.in_schedule(OnEnter(DayState::Evening)))
		.add_system(spawn_drop_zone.in_schedule(OnEnter(DayState::Night)))
		// Despawning systems
		.add_systems(despawn_systems.in_schedule(OnExit(GameState::Game)))
		.add_systems(despawn_systems.in_schedule(OnExit(DayState::Morning)))
		.add_systems(despawn_systems.in_schedule(OnExit(DayState::Evening)))
		.add_systems(despawn_systems.in_schedule(OnExit(DayState::Night)))
		;
	}
}

#[derive(Resource)]
pub struct DropZone {
	pub max_lim: Vec3,
	pub min_lim: Vec3,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct DropSprite;

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct HangTimer(pub Timer);

fn spawn_drop_zone(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(500.0, 725.0, DROPZONE_DEPTH)
				.with_scale(Vec3::new(1.0/32.0*1000.0, 1.0/32.0*150.0, 0.0)),
				texture: asset_server.load("sprites/dropzone.png"),
				sprite: Sprite {
					color: Color::rgba(1.0, 1.0, 1.0, 0.6),
					..default()
				},
				..default()
			},
			DropSprite,
		)
	);
}

fn spawn_ball(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
	asset_server: Res<AssetServer>,
	drop_zone: Res<DropZone>,
	state: Res<State<DayState>>,
	mut user_data: ResMut<UserData>,
	mut money_timer: ResMut<OutOfMoneyTimer>,
	time: Res<Time>,
	mut next_day_state: ResMut<NextState<DayState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut pkv: ResMut<PkvStore>,
) {
	let window = window_query.get_single().unwrap();

	if mouse.just_pressed(MouseButton::Left) {
		if let Some(current_pos) = window.cursor_position() {
			if current_pos.x < drop_zone.max_lim.x && current_pos.x > drop_zone.min_lim.x 
				&& current_pos.y < drop_zone.max_lim.y && current_pos.y > drop_zone.min_lim.y {
				let drop_type = match state.0 {
					DayState::Dawn => DropType::Ball,
					DayState::Morning => rand::random(),
					DayState::Evening => DropType::Mouth,
					DayState::Night => DropType::Ball,
				};
				match drop_type {
					DropType::Ball => {
						if user_data.money >= 3.0 {user_data.money -= 3.0;
							commands.spawn(
								(
									SpriteBundle {
										transform: Transform::from_xyz(current_pos.x, current_pos.y, BALL_DEPTH),
										texture: asset_server.load(drop_type.get_path()),
										..default()
									},
									Ball,
									Held,
									drop_type,
								)
							);
						} else {
							/*money_timer.0.tick(time.delta());
							let mut flag = false;
							if money_timer.0.finished() {
								flag = true;
							}
							if flag {
								let next =  DayState::Morning;

								if let Ok(mut save_data) = pkv.get::<SaveData>("user_info") {
									// Save progress here
									save_data.money = user_data.money;
									save_data.stress = user_data.stress;
									save_data.flirt = user_data.flirt;
									save_data.royal = user_data.royal;
									save_data.drugs_taken = user_data.drugs_taken;
						
									user_data.day = user_data.day + 1.0;
									save_data.day = user_data.day;
									if save_data.royal > 50.0 {
										save_data.royal -= 50.0;
										user_data.royal -= 50.0;
									} else if save_data.royal > 20.0 {
										save_data.royal -= 10.0;
										user_data.royal -= 10.0;
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
							}*/
							//You're out of luck
							//You're out of balls
						};
					}
					_ => {
						commands.spawn(
							(
								SpriteBundle {
									transform: Transform::from_xyz(current_pos.x, current_pos.y, BALL_DEPTH),
									texture: asset_server.load(drop_type.get_path()),
									..default()
								},
								Ball,
								Held,
								drop_type,
							)
						);
					}
				}
			}
		}
	}
}

fn despawn_ball(
	mut commands: Commands,
	ball_query: Query<(Entity, &Transform, &DropType), With<Ball>>,
) {
	for (entity, &transform, _) in ball_query.iter() {
		if transform.translation.y <= 0.0 {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn launch_ball(
	mut commands: Commands,
	ball_query: Query<(Entity, &Transform), With<Held>>,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
) {
	let window = window_query.get_single().unwrap();
	for (entity, transform) in ball_query.iter() {
		if mouse.just_released(MouseButton::Left) {
			if let Some(current_pos) = window.cursor_position() {

				commands.entity(entity).remove::<Held>();
				commands.entity(entity).insert(Velocity(Vec2::new(
					(transform.translation.x - current_pos.x) * 4.0, 
					(transform.translation.y - current_pos.y) * 4.0)
					.clamp_length(0.0, 512.0)));
			}
		}
	}
}
