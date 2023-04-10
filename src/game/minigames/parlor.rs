use bevy::prelude::*;

use crate::{
	game::{
		physics::{Velocity, BallTargetHit, BallCollisionEvent, BallCollisionEventType}, PachinkoSystemSet,
		pegs::{Peg, PEG_DEPTH, PegType}, targets::ClearOnDayTransition, food::DropType,
		states::{DayState, PauseState, GameState}, balls::{BALL_DEPTH, Ball}
	}, setup::UserData
};

pub const PRIZE_TIME: f32 = 10.0;
pub const PRIZE_LAUNCH_TIME: f32 = 0.5;

pub const HORIZONTAL_SPEED: f32 = 30.0;
pub const VERTICAL_SPEED: f32 = 12.0;
pub struct ParlorMinigame;

impl Plugin for ParlorMinigame {
	fn build(&self, app: &mut App) {
		app
		.insert_resource(PrizeTimer(Timer::from_seconds(PRIZE_TIME, TimerMode::Once)))
		.insert_resource(PrizeLaunchTimer(Timer::from_seconds(PRIZE_LAUNCH_TIME, TimerMode::Repeating)))
		.add_system(handle_parlor_balls
			.run_if(on_event::<BallTargetHit>())
			.run_if(in_state(DayState::Night))			
			.in_set(PachinkoSystemSet))
		.add_system(ring_bells
			.run_if(on_event::<BallCollisionEvent>())
			.run_if(in_state(DayState::Night))			
			.in_set(PachinkoSystemSet))
		.add_system(spawn_parlor_pegs.in_schedule(OnEnter(DayState::Night)))
		.add_system(spawn_bells.in_schedule(OnEnter(DayState::Night)))
		.add_system(spawn_prize
			.run_if(in_state(DayState::Night))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_system(move_drug_pegs
			.run_if(in_state(DayState::Night))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_system(move_pegs_horizontal
			.run_if(in_state(DayState::Night))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_system(move_pegs_vertical
			.run_if(in_state(DayState::Night))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		;
	}
}

#[derive(Resource)]
pub struct PrizeTimer(pub Timer);

#[derive(Resource)]
pub struct PrizeLaunchTimer(pub Timer);

#[derive(Component)]
pub struct DrugPeg(pub f32, pub bool);

#[derive(Component)]
pub struct MoveHorizontalPeg(pub f32, pub bool);

#[derive(Component)]
pub struct MoveVerticalPeg(pub f32, pub bool);

#[derive(Component)]
pub struct Bell(bool);

fn shallow_triangle(
	top_x: f32,
	top_y: f32,
) -> Vec<(f32, f32)> {
	vec![(top_x, top_y),(top_x-25.0,top_y-20.0),(top_x-50.0,top_y-40.0),(top_x+25.0,top_y-20.0),(top_x+50.0,top_y-40.0)]
}

fn spawn_parlor_pegs(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Standard Triangles
	let triangle_list = vec![
		// Two pit dividers
		(350.0,88.0),(650.0,88.0),
		// Mid level dividers
		(200.0,150.0),(800.0,150.0),
		// Top level dividers
		(150.0,600.0),(500.0,600.0),(850.0,600.0),
		];

	// Standard Pegs
	let mut single_list: Vec<(f32, f32)> = vec![
		(265.0,600.0),(392.0,600.0),(616.0,600.0),(728.0,600.0),
		(42.0,408.0),(958.0,408.0),
		(330.0,340.0),(410.0,340.0),(500.0,340.0),(590.0,340.0),(670.0,340.0),
		(42.0,264.0),(958.0,264.0),
		(360.0,200.0),(640.0,200.0),
		(100.0,340.0),(170.0,340.0),(830.0,340.0),(900.0,340.0),
		(88.0,48.0),(60.0,76.0),(32.0,104.0), (912.0,48.0),(940.0,76.0),(968.0,104.0),
		(920.0,16.0),(680.0,16.0),(620.0,16.0),(380.0,16.0),(320.0,16.0),(80.0,16.0),
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

	// Drug Pegs
	let map = shallow_triangle(500.0, 190.0);

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
				DrugPeg(loc.1, true)
			)
		);
	}

	// Horizontal Triangles
	let triangle_list = vec![
		// High level movers
		(700.0,500.0),(300.0,500.0)
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
	let map: Vec<(f32, f32)> = vec![];

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

fn spawn_bells(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	let map = vec![
		// Top Bell
		(500.0,500.0),
		// Center Bells
		(250.0,350.0),(750.0,350.0),
		// Corner Bells
		(100.0,200.0),(900.0,200.0),
		// Drug Bell
		(500.0,150.0),];

	let bell_texture_handle = asset_server.load("droppables/bell.png");

	for loc in map {
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: bell_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(DropType::Bell)),
				ClearOnDayTransition,
				Bell(false),
			)
		);
	}
}

fn ring_bells(
	mut commands: Commands,
	mut collision_events: EventReader<BallCollisionEvent>,
	asset_server: Res<AssetServer>,
	mut user_data: ResMut<UserData>,
) {
	for collision in collision_events.iter() {
		match collision.collision_type {
			BallCollisionEventType::Ball => (),
			BallCollisionEventType::Peg{peg_type: PegType::ItemPeg(drop_type), peg, ..} => {
				if drop_type == DropType::Bell {
					commands.entity(peg).remove::<Bell>();
					commands.entity(peg).remove::<bevy::asset::Handle<Image>>();
					commands.entity(peg).insert(Bell(true));
					let sprite: bevy::asset::Handle<Image> = asset_server.load("droppables/crown.png");
					commands.entity(peg).insert(sprite.clone());
					user_data.money = (user_data.money + 2.0).clamp(0.0, 100.0);
				}
			},
			BallCollisionEventType::Peg{..} => (),
		}
	}
}

fn spawn_prize(
	mut commands: Commands,
	mut bell_query: Query<(Entity, &mut Bell)>,
	time: Res<Time>,
	mut prize_timer: ResMut<PrizeTimer>,
	mut prize_launch_timer: ResMut<PrizeLaunchTimer>,
	asset_server: Res<AssetServer>,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut next_day_state: ResMut<NextState<DayState>>,
) {
	let mut total_bells_rang = 0;
	for (_, bell) in bell_query.iter() {
		if bell.0 {
			total_bells_rang += 1;
		}
	}
	if total_bells_rang == 6 {
		prize_timer.0.tick(time.delta());
		prize_launch_timer.0.tick(time.delta());
		if prize_launch_timer.0.just_finished() {
			let (x_rand,y_rand): (f32,f32) = rand::random();
			let prize_chance = rand::random::<f32>();
			let prize = if prize_chance > 0.3 {DropType::Diamond
				} else if prize_chance > 0.6 {DropType::Crown
				} else {DropType::Money};
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(1300.0, 700.0, BALL_DEPTH),
						texture: asset_server.load(prize.get_path()),
						..default()
					},
					Ball,
					Velocity(Vec2::new(-(300.0 + x_rand * 150.0), 250.0 + y_rand * 50.0)),
					prize,
				)
			);
			if prize_timer.0.finished() {
				for (entity, mut bell) in bell_query.iter_mut() {
					bell.0 = false;
					commands.entity(entity).remove::<bevy::asset::Handle<Image>>();
					let sprite: bevy::asset::Handle<Image> = asset_server.load("droppables/bell.png");
					commands.entity(entity).insert(sprite.clone());

					next_day_state.0 = Some(DayState::Dawn);
					next_pause_state.0 = Some(PauseState::Initial);
					next_game_state.0 = Some(GameState::Outro);
					prize_timer.0.reset();
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
		if move_peg.1 {transform.translation.x += HORIZONTAL_SPEED * time.delta_seconds()
			} else {transform.translation.x += -HORIZONTAL_SPEED * time.delta_seconds()};
		if transform.translation.x >= move_peg.0 + 125.0 {move_peg.1 = false};
		if transform.translation.x <= move_peg.0 - 125.0 {move_peg.1 = true};
	}
}

fn move_pegs_vertical(
	mut vertical_peg_query: Query<(&mut Transform, &mut MoveVerticalPeg)>,
	time: Res<Time>,
) {
	for (mut transform, mut move_peg) in vertical_peg_query.iter_mut() {
		if move_peg.1 {transform.translation.y += VERTICAL_SPEED * time.delta_seconds()
			} else {transform.translation.y += -VERTICAL_SPEED * time.delta_seconds()};
		if transform.translation.y >= move_peg.0 + 120.0 {move_peg.1 = false};
		if transform.translation.y <= move_peg.0 {move_peg.1 = true};
	}
}

fn move_drug_pegs(
	mut drug_peg_query: Query<(&mut Transform, &mut DrugPeg)>,
	user_data: Res<UserData>,
	time: Res<Time>,
) {
	if user_data.royal > 50.0 {
		for (mut transform, mut drug_peg) in drug_peg_query.iter_mut() {
			if drug_peg.1 {transform.translation.y += VERTICAL_SPEED * time.delta_seconds()
				} else {transform.translation.y += -VERTICAL_SPEED * time.delta_seconds()};
			if transform.translation.y >= drug_peg.0 + 120.0 {drug_peg.1 = false};
			if transform.translation.y <= drug_peg.0 {drug_peg.1 = true};
		}
	}
}

fn handle_parlor_balls(
//	mut commands: Commands,
	mut collision_events: EventReader<BallTargetHit>,
//	ball_query: Query<(Entity, &Transform, &DropType), With<Ball>>,
	mut user_data: ResMut<UserData>,
) {
	for ball_event in collision_events.iter() {
		let _target_entity = ball_event.target;
		let _ball_type = ball_event.ball_type;
		user_data.money = (user_data.money + 1.0).clamp(0.0, 100.0)
	}
}
