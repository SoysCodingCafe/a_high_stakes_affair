use bevy::{prelude::*, math::Vec3Swizzles, window::PrimaryWindow};

use crate::{
	setup::UserData,
	game::{
		physics::{BallTargetHit, Velocity}, 
		food::DropType, PachinkoSystemSet, states::{DayState, GameState, PauseState},
		targets::{LinkedBaskets, Target, BOTTOM_TARGET_DEPTH, LABEL_TARGET_DEPTH, TOP_TARGET_DEPTH, ClearOnDayTransition},
		balls::{Ball, DropZone, BALL_DEPTH, Held, HangTimer}, pegs::{Peg, PEG_DEPTH}
	}
};

use super::parlor::{MoveHorizontalPeg, MoveVerticalPeg, HORIZONTAL_SPEED, VERTICAL_SPEED};

pub const HANG_TIME: f32 = 10.0;
pub const SPAWN_TIME: f32 = 1.0;

pub struct WorkMinigame;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct WorkSystemSet;

impl Plugin for WorkMinigame {
	fn build(&self, app: &mut App) {
		app
		.configure_set(WorkSystemSet
			.run_if(in_state(DayState::Morning))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_system(handle_work_balls
			.run_if(on_event::<BallTargetHit>())
			.run_if(in_state(DayState::Morning))
			.in_set(PachinkoSystemSet))
		.add_system(move_targets
			.in_set(WorkSystemSet)
			.before(PachinkoSystemSet))
		.add_systems((spawn_work_ball, drop_work_ball, grab_work_ball).in_set(WorkSystemSet))
		.add_system(initialise_spawn_timer.in_schedule(OnEnter(DayState::Morning)))
		.add_system(spawn_work_pegs.in_schedule(OnEnter(DayState::Morning)))
		.add_system(spawn_storage.in_schedule(OnEnter(DayState::Morning)))
		.add_system(store_balls.in_set(WorkSystemSet))
		.add_system(move_pegs_horizontal.in_set(WorkSystemSet))
		.add_system(move_pegs_vertical.in_set(WorkSystemSet))
		;
	}
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct Storage;

#[derive(Component)]
pub struct Basket;

#[derive(Component)]
pub struct Handle;

pub fn initialise_spawn_timer(
	mut commands: Commands,
) {
	commands.insert_resource(SpawnTimer(Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)));
}

fn shallow_triangle(
	top_x: f32,
	top_y: f32,
) -> Vec<(f32, f32)> {
	vec![(top_x, top_y),(top_x-25.0,top_y-20.0),(top_x-50.0,top_y-40.0),(top_x+25.0,top_y-20.0),(top_x+50.0,top_y-40.0)]
}

fn spawn_work_pegs(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Standard Triangles
	let triangle_list = vec![
		// Top level dividers
		(150.0,628.0),(500.0,628.0),(850.0,628.0),
		];

	// Standard Pegs
	let mut single_list: Vec<(f32, f32)> = vec![
		// Right buffer
		(960.0,16.0),(976.0,56.0),(984.0,96.0),
		// Left buffer
		(40.0,16.0),(24.0,56.0),(16.0,96.0),
		// Inter basket buffer
		(220.0,62.0),(408.0,62.0),(592.0,62.0),(780.0,62.0),
		(220.0,24.0),(408.0,24.0),(592.0,24.0),(780.0,24.0),
		// Random
		(900.0,296.0),(680.0,296.0),(500.0,296.0),(320.0,296.0),(100.0,296.0),
		(200.0,460.0),(400.0,460.0),(600.0,460.0),(800.0,460.0),
		];
	// Left Wall
	for i in 0..11 {
		single_list.push((16.0, i as f32*48.0 + 144.0));
	}
	// Right Wall
	for i in 0..11 {
		single_list.push((984.0, i as f32*48.0 + 144.0));
	}

	let mut map: Vec<(f32, f32)> = vec![];
	for coord in triangle_list {
		map.append(&mut shallow_triangle(coord.0, coord.1));
	}
	map.append(&mut single_list);

	let peg_texture_handle = asset_server.load("sprites/peg.png");

	for loc in map {
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg::default(),
				ClearOnDayTransition,
			)
		);
	}

	// Horizontal Triangles
	let triangle_list = vec![
		// High level movers
		(700.0,400.0),(300.0,400.0)
		];

	// Horizontal Pegs
	let mut single_list: Vec<(f32, f32)> = vec![];

	let mut map: Vec<(f32, f32)> = vec![];
	for coord in triangle_list {
		map.append(&mut shallow_triangle(coord.0, coord.1));
	}
	map.append(&mut single_list);

	let peg_texture_handle = asset_server.load("sprites/peg.png");

	for loc in map {
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg::default(),
				ClearOnDayTransition,
				MoveHorizontalPeg(loc.0, true)
			)
		);
	}

	// Vertical Pegs
	let map = vec![
		(220.0,100.0),(200.0,132.0),(220.0,164.0),(240.0,132.0),
		(408.0,100.0),(388.0,132.0),(408.0,164.0),(428.0,132.0),
		(592.0,100.0),(572.0,132.0),(592.0,164.0),(612.0,132.0),
		(780.0,100.0),(760.0,132.0),(780.0,164.0),(800.0,132.0),];

	let peg_texture_handle = asset_server.load("sprites/peg.png");

	for loc in map {
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg::default(),
				ClearOnDayTransition,
				MoveVerticalPeg(loc.1, true)
			)
		);
	}
}

fn handle_work_balls(
	mut commands: Commands,
	mut collision_events: EventReader<BallTargetHit>,
	mut user_data: ResMut<UserData>,
	basket_query: Query<(Entity, &LinkedBaskets, &Transform, &DropType, &Velocity, &Target)>,
	asset_server: Res<AssetServer>,
) {
	for ball_event in collision_events.iter() {
		let target_entity = ball_event.target;
		let ball_type = ball_event.ball_type;
		if let Ok((basket_entity, &linked_baskets, &basket_transform, &basket_type, &basket_velocity, &basket_target)) = basket_query.get(target_entity) {
			if basket_type == ball_type {
				user_data.money = (user_data.money + 10.0).clamp(0.0, 100.0);
				let top_entity = commands.spawn(
					(
						SpriteBundle {
							transform: Transform::from_xyz(basket_transform.translation.x, 32.0, TOP_TARGET_DEPTH),
							texture: asset_server.load("sprites/handle.png"),
							..default()
						},
						basket_velocity,
						//basket_target,
						Handle,
						ClearOnDayTransition
					)
				).id();
				let bottom_entity = commands.spawn(
					(
						SpriteBundle {
							transform: Transform::from_xyz(basket_transform.translation.x, 32.0, BOTTOM_TARGET_DEPTH),
							texture: asset_server.load("sprites/basket.png"),
							..default()
						},
						basket_velocity,
						//basket_target,
						Basket,
						ClearOnDayTransition
					)
				).id();
				let drop_type: DropType = rand::random();
				commands.spawn(
					(
						SpriteBundle {
							transform: Transform::from_xyz(basket_transform.translation.x, 16.0, LABEL_TARGET_DEPTH),
							texture: asset_server.load(drop_type.get_path()),
							..default()
						},
						basket_velocity,
						drop_type,
						basket_target,
						LinkedBaskets((top_entity, bottom_entity)),
						ClearOnDayTransition
					)
				);
				commands.entity(linked_baskets.0.0).despawn_recursive();
				commands.entity(linked_baskets.0.1).despawn_recursive();
				commands.entity(basket_entity).despawn_recursive();
			} else {
				//println!("Incorrect Basket");
				user_data.money = (user_data.money + 1.0).clamp(0.0, 100.0);
			}
			user_data.stress = (user_data.stress + 5.0).clamp(0.0, 100.0)
		} else {
			// TODO: handle, this is probably balls falling in storage
			warn!("Balls are falling into targets which are inapropriate for current daytime. (Ball fell into non-basket)");
		}
	}
}

fn move_targets(
	mut storage_query: Query<(&Transform, &mut Velocity), With<Storage>>,
	time: Res<Time>,
) {
	for (transform, mut velocity) in storage_query.iter_mut() {
		let target = transform.translation.x + velocity.0.x * time.delta_seconds();
		if target > 850.0 && velocity.0.x > 0.0 || target < 150.0 && velocity.0.x < 0.0 {
			velocity.0.x = -velocity.0.x;
		}
	}
}

fn spawn_storage(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(500.0, 525.0, LABEL_TARGET_DEPTH),
				texture: asset_server.load("sprites/storage.png"),
				..default()
			},
			Storage,
			Velocity(Vec2::new(100.0, 0.0)),
			ClearOnDayTransition,
		)
	);
}

fn store_balls(
	mut commands: Commands,
	mut ball_query: Query<(Entity, &Transform), With<Ball>>,
	storage_query: Query<&Transform, With<Storage>>,
) {
	let storage_transform = storage_query.single();
	for (entity, ball_transform) in ball_query.iter_mut() {
		if ball_transform.translation.x > storage_transform.translation.x - 50.0 
		&& ball_transform.translation.x < storage_transform.translation.x + 50.0 
		&& ball_transform.translation.y > storage_transform.translation.y - 25.0 
		&& ball_transform.translation.y < storage_transform.translation.y {
			commands.entity(entity).despawn_recursive();
		}
	}
}

fn spawn_work_ball(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	drop_zone: Res<DropZone>,
	ball_query: Query<(&Transform, With<Ball>)>,
	mut spawn_timer: ResMut<SpawnTimer>,
	time: Res<Time>,
	user_data: Res<UserData>,
) {
	spawn_timer.0.tick(time.delta() / (user_data.royal/10.0).clamp(1.0, 10.0) as u32);

	if spawn_timer.0.just_finished() {
		let x_rand: f32 = rand::random();
		let y_rand: f32 = rand::random();
		let spawn_transform = Vec2::new(
			x_rand * (drop_zone.max_lim.x - drop_zone.min_lim.x - 32.0) + drop_zone.min_lim.x + 16.0,
			y_rand * (drop_zone.max_lim.y - drop_zone.min_lim.y - 32.0) + drop_zone.min_lim.y + 16.0,
		);
		let mut collision = false;
		for (transform, _) in ball_query.iter() {
			if (transform.translation.xy() - spawn_transform).length_squared() < 32.0 * 32.0 {
				collision = true;
			}
		}
		if !collision {
			let drop_type: DropType = rand::random();
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(spawn_transform.x, spawn_transform.y, BALL_DEPTH),
						texture: asset_server.load(drop_type.get_path()),
						..default()
					},
					Ball,
					HangTimer(Timer::from_seconds(HANG_TIME, TimerMode::Once)),
					drop_type,
				)
			);
		}
	}
}

fn drop_work_ball (
	mut commands: Commands,
	mut hang_timer_query: Query<(Entity, &mut HangTimer)>,
	time: Res<Time>,
	user_data: Res<UserData>,
) {
	for (entity, mut hang_timer) in hang_timer_query.iter_mut() {
		hang_timer.0.tick(time.delta() / (user_data.royal/20.0).clamp(1.0, 5.0) as u32);
		if hang_timer.0.just_finished() {
			commands.entity(entity).remove::<HangTimer>();
			commands.entity(entity).insert(Velocity(Vec2::splat(0.0)));
		}
	}
}

fn grab_work_ball (
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	mouse: Res<Input<MouseButton>>,
	ball_query: Query<(Entity, &Transform, With<HangTimer>)>
) {
	let window = window_query.get_single().unwrap();
	for (entity, transform, _) in ball_query.iter() {
		if mouse.just_pressed(MouseButton::Left) {
			if let Some(current_pos) = window.cursor_position() {
				if (current_pos - transform.translation.xy()).length_squared() < 16.0 * 16.0 {
					commands.entity(entity).remove::<HangTimer>();
					commands.entity(entity).insert(Held);
				}
			}
		}
	}
}

fn move_pegs_horizontal(
	mut horizontal_peg_query: Query<(&mut Transform, &mut MoveHorizontalPeg)>,
	time: Res<Time>,
) {
	for (mut transform, mut move_peg) in horizontal_peg_query.iter_mut() {
		if move_peg.1 {transform.translation.x += HORIZONTAL_SPEED * 5.0 * time.delta_seconds()
			} else {transform.translation.x += -HORIZONTAL_SPEED * 5.0 * time.delta_seconds()};
		if transform.translation.x >= move_peg.0 + 125.0 {move_peg.1 = false};
		if transform.translation.x <= move_peg.0 - 125.0 {move_peg.1 = true};
	}
}

fn move_pegs_vertical(
	mut vertical_peg_query: Query<(&mut Transform, &mut MoveVerticalPeg)>,
	time: Res<Time>,
) {
	for (mut transform, mut move_peg) in vertical_peg_query.iter_mut() {
		if move_peg.1 {transform.translation.y += VERTICAL_SPEED * 6.0 * time.delta_seconds()
			} else {transform.translation.y += -VERTICAL_SPEED * 6.0 * time.delta_seconds()};
		if transform.translation.y >= move_peg.0 + 120.0 {move_peg.1 = false};
		if transform.translation.y <= move_peg.0 {move_peg.1 = true};
	}
}