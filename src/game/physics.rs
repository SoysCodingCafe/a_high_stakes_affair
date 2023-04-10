// Responsible for the physical simulation (collisions and kinematics)
// Doesnt spawn/despawn anything, only moves it

use bevy::{prelude::*, math::Vec3Swizzles};

use super::{balls::{Ball, HangTimer}, pegs::{Peg, PegType}, states::Frame, PachinkoSystemSet, targets::Target, food::DropType};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app
		.add_event::<BallCollisionEvent>()
		.add_event::<BallTargetHit>()
		// Movement systems
		.add_systems((
			apply_gravity,
			move_ball,
			ball_ball_collide,
			ball_peg_collide,
			ball_frame_collide,
			ball_target_collide,
			ball_out_of_bounds,
		).chain().in_set(PachinkoSystemSet));
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct BallCollisionEvent {
	pub pos: Vec2,
	pub collision_type: BallCollisionEventType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BallCollisionEventType {
	Peg {
		ball_type: Option<DropType>,
		peg_type: PegType,
		ball: Entity,
		peg: Entity,
	},
	Ball
}

#[derive(Clone, Copy, PartialEq)]
pub struct BallTargetHit {
	pub ball_type: DropType,
	pub target: Entity,
}

#[derive(Component, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Resource)]
pub struct Gravity(pub f32);

pub fn move_ball(
	mut ball_query: Query<(&mut Transform, &Velocity), Without<HangTimer>>,
	time: Res<Time>,
	//user_data: Res<UserData>,
) {
	for (mut transform, velocity) in ball_query.iter_mut() {
		transform.translation += (velocity.0 * time.delta_seconds()).extend(0.0);
		//transform.translation += (velocity.0 * time.delta_seconds() * (1.0 - (user_data.royal as f32 / 100.0)).clamp(0.2, 1.0)).extend(0.0);
	}
}

fn apply_gravity(
	mut velocity_query: Query<&mut Velocity, (With<Ball>, Without<HangTimer>)>,
	time: Res<Time>,
	gravity: Res<Gravity>,
	//user_data: Res<UserData>,
) {
	for mut velocity in velocity_query.iter_mut() {
		velocity.0.y += gravity.0 * time.delta_seconds();
		//velocity.0.y += gravity.0 * time.delta_seconds() * (1.0 - (user_data.royal as f32 / 100.0)).clamp(0.2, 1.0);
	}
}

fn ball_out_of_bounds(
	mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
	for (mut transform, mut velocity) in ball_query.iter_mut() {
		if transform.translation.x <= 16.0 || transform.translation.x >= 1584.0 {
			let push = if velocity.0.x.is_sign_negative() {
				16.0
			} else {
				-16.0
			};
			transform.translation.x += push;
			velocity.0.x = -velocity.0.x * 0.8;
		}
	}
}

fn ball_frame_collide(
	mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
	mut frame_query: Query<&Transform, (With<Frame>, Without<Ball>)>,
) {
	for frame_transform in frame_query.iter_mut() {
		for (mut ball_transform, mut velocity) in ball_query.iter_mut() {
			let rect_right = frame_transform.translation.x + 100.0;
			let rect_left = frame_transform.translation.x - 100.0;
			let rect_top = frame_transform.translation.y + 24.0;
			let rect_bottom = frame_transform.translation.y - 24.0;

			let closest_x = ball_transform.translation.x.clamp(rect_left, rect_right);
			let closest_y = ball_transform.translation.y.clamp(rect_bottom, rect_top);

			let distance_x = ball_transform.translation.x - closest_x;
			let distance_y = ball_transform.translation.y - closest_y;
			let distance = (distance_x.powf(2.0) + distance_y.powf(2.0)).sqrt();

			if distance < 16.0 {
				let depth = 16.0 - distance;
				let angle = distance_y.atan2(distance_x);
				ball_transform.translation.x += angle.cos() * depth;
				ball_transform.translation.y += angle.sin() * depth;
				
				// Bottom 0 -1
				// Top 0 1
				// Left -1 0
				// Right 1 0
				match (angle.cos().round() as i8, angle.sin().round() as i8) {
					(0,1) => {velocity.0.y = -(velocity.0.y * 0.6); velocity.0.x *= 0.6},
					(0,-1) => {velocity.0.y = -(velocity.0.y * 0.6); velocity.0.x *= 0.6},
					(1,0) => {velocity.0.x = -(velocity.0.x * 0.6); velocity.0.y *= 0.6},
					(-1,0) => {velocity.0.x = -(velocity.0.x * 0.6); velocity.0.y *= 0.6},
					(_,_) => (),
				}
			}
		}
	}
}

fn ball_peg_collide(
	mut ball_query: Query<(Entity, &mut Transform, &mut Velocity, Option<&DropType>), With<Ball>>,
	peg_query: Query<(Entity, &Transform, &Peg), Without<Ball>>,
	time: Res<Time>,
	mut collision_events: EventWriter<BallCollisionEvent>,
) {
	let bounciness = 80.0; //50.0 is normalish, 100.0 very bouncy
	for (ball_entity, mut ball_transform, mut velocity, drop_type) in ball_query.iter_mut() {
		for (peg_entity, &peg_transform, &peg) in peg_query.iter() {
			let offset = (ball_transform.translation - peg_transform.translation).xy();
			if offset.length_squared() <= 32.0 * 32.0 {
				let dp = offset * Vec2::dot(velocity.0, offset) / ((offset.length_squared()));
				velocity.0 -= dp * bounciness * time.delta_seconds();
				let push = offset.normalize() * 1.01 * 32.0 - offset;
				ball_transform.translation += push.extend(0.0);
				collision_events.send(BallCollisionEvent {
					pos: ball_transform.translation.xy(),
					collision_type: BallCollisionEventType::Peg {
						ball_type: drop_type.copied(),
						peg_type: peg.0,
						ball: ball_entity,
						peg: peg_entity
					},
				});
			}
		}
	}
}

fn ball_ball_collide(
	mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
	time: Res<Time>,
	mut collision_events: EventWriter<BallCollisionEvent>,
) {
	let bounciness = 50.0; //30.0 is normalish, 80.0 very bouncy
	let mut iter = ball_query.iter_combinations_mut();
	while let Some([
		(mut transform_a, mut velocity_a),
		(mut transform_b, mut velocity_b)
	]) = iter.fetch_next() {
		let offset = (transform_a.translation - transform_b.translation).xy();
		if offset.length_squared() <= 32.0 * 32.0 {
			let relative_velocity = velocity_a.0 - velocity_b.0;
			let dp = offset*Vec2::dot(relative_velocity, offset) / ((offset.length_squared()));

			velocity_a.0 -= dp * bounciness * time.delta_seconds();
			velocity_b.0 += dp * bounciness * time.delta_seconds();

			let push = offset.normalize() * 1.01 * 32.0 - offset;
			transform_a.translation += push.extend(0.0);
			transform_b.translation -= push.extend(0.0);
			collision_events.send(BallCollisionEvent {
				pos: 0.5 * (transform_a.translation + transform_b.translation).xy(),
				collision_type: BallCollisionEventType::Ball,
			});
		}
	}
}

pub fn ball_target_collide(
	mut ball_query: Query<(Entity, &Transform, &DropType), With<Ball>>,
	target_query: Query<(Entity, &Transform, &Target)>,
	mut events: EventWriter<BallTargetHit>
) {
	for (_entity, &transform, &drop_type) in ball_query.iter_mut() {
		if transform.translation.y <= 0.0 {
			//println!("Ball hit the bottom!");
			for (target_entity, &target_transform, &target) in target_query.iter() {
				let distance = (target_transform.translation.x - transform.translation.x).abs();
				//println!("Distance: {}", distance);
				if distance < target.radius {
					//println!("Ball event sent!");
					events.send(BallTargetHit { ball_type: drop_type, target: target_entity });
				}
			}
		}
	}
}
