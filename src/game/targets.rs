use bevy::prelude::*;

use crate::game::{states::DayState, physics::Velocity, food::DropType, minigames::{work::{Basket, Handle}, food::Soup}};

pub const TOP_TARGET_DEPTH: f32 = -1.5;
pub const BOTTOM_TARGET_DEPTH: f32 = -0.5;
pub const LABEL_TARGET_DEPTH: f32 = -0.02;

pub struct TargetsPlugin;

impl Plugin for TargetsPlugin {
	fn build(&self, app: &mut App) {
		app
		// Spawning systems
		.add_system(spawn_targets.in_schedule(OnEnter(DayState::Morning)))
		.add_system(spawn_targets.in_schedule(OnEnter(DayState::Evening)))
		.add_system(spawn_targets.in_schedule(OnEnter(DayState::Night)))
		;
	}
}

#[derive(Component)]
pub struct ClearOnDayTransition;

#[derive(Component)]
pub struct Bowl;

#[derive(Component)]
pub struct Rim;

#[derive(Component)]
pub struct Hole;

#[derive(Component, PartialEq, Clone, Copy)]
pub struct Target {
	pub radius: f32
}

#[derive(Component, Clone, Copy)]
pub struct LinkedBaskets(pub (Entity, Entity));

struct TargetDetails {
	amount: u8,
	width: f32,
	height: f32,
	spacing: f32,
	speed: f32,
	top_sprite: String,
	bottom_sprite: String,
}

fn spawn_targets(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	current_state: Res<State<DayState>>,
) {
	let target_details = match current_state.0 {
		DayState::Dawn => TargetDetails {
			amount: 0,
			width: 0.0,
			height: 0.0,
			spacing: 0.0,
			speed: 0.0,
			top_sprite: "".to_string(),
			bottom_sprite: "".to_string(),
		},
		DayState::Morning => TargetDetails {
			amount: 5,
			width: 128.0,
			height: 32.0,
			spacing: 60.0,
			speed: 0.0,
			top_sprite: "sprites/handle.png".to_string(),
			bottom_sprite: "sprites/basket.png".to_string(),
		},
		DayState::Evening => TargetDetails {
			amount: 1,
			width: 300.0,
			height: 25.0,
			spacing: 500.0,
			speed: 100.0,
			top_sprite: "sprites/soup.png".to_string(),
			bottom_sprite: "sprites/bowl.png".to_string(),
		},
		DayState::Night => TargetDetails {
			amount: 3,
			width: 200.0,
			height: 25.0,
			spacing: 100.0,
			speed: 0.0,
			top_sprite: "sprites/rim.png".to_string(),
			bottom_sprite: "sprites/hole.png".to_string(),
		},
	};
	for i in 0..target_details.amount.clone() {
		let top_entity = commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(
						i as f32 * (target_details.spacing + target_details.width) + target_details.spacing + target_details.width/2.0, 
						target_details.height, 
						TOP_TARGET_DEPTH),
					texture: asset_server.load(target_details.top_sprite.clone()),
					sprite: Sprite {
						..default()
					},
					..default()
				},
				ClearOnDayTransition,
				Velocity(Vec2::new(target_details.speed, 0.0)),
			)
		).id();

		match current_state.0 {
			DayState::Dawn => (),
			DayState::Morning => {commands.entity(top_entity).insert(Handle);},
			DayState::Evening => {commands.entity(top_entity).insert(Soup);commands.entity(top_entity).insert(Target{radius: 0.5 * target_details.width});},
			DayState::Night => {commands.entity(top_entity).insert(Rim);commands.entity(top_entity).insert(Target{radius: 0.5 * target_details.width});},
		}

		let bottom_entity = commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(
						i as f32 * (target_details.spacing + target_details.width) + target_details.spacing + target_details.width/2.0, 
						target_details.height, 
						BOTTOM_TARGET_DEPTH),
					texture: asset_server.load(target_details.bottom_sprite.clone()),
					sprite: Sprite {
						..default()
					},
					..default()
				},
				ClearOnDayTransition,
				Velocity(Vec2::new(target_details.speed, 0.0)),
			)
		).id();

		match current_state.0 {
			DayState::Dawn => (),
			DayState::Morning => {commands.entity(bottom_entity).insert(Basket);},
			DayState::Evening => {commands.entity(bottom_entity).insert((Bowl, Soup));},
			DayState::Night => {commands.entity(bottom_entity).insert(Hole);},
		}

		if current_state.0 == DayState::Morning {
			let drop_type: DropType = rand::random();
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(i as f32 * (target_details.spacing + target_details.width) + target_details.spacing + target_details.width/2.0, 16.0, LABEL_TARGET_DEPTH),
						texture: asset_server.load(drop_type.get_path()),
						..default()
					},
					ClearOnDayTransition,
					Velocity(Vec2::new(target_details.speed, 0.0)),
					drop_type,
					Target {radius: 0.5 * target_details.width},
					LinkedBaskets((top_entity, bottom_entity))
				)
			);
		}
	}
}