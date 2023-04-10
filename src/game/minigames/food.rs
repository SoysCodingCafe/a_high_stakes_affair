use bevy::prelude::*;

use crate::{
	game::{
		physics::{BallTargetHit, Velocity, BallCollisionEvent, BallCollisionEventType},
		food::{DropCategory, random_inedible_except_special, random_non_drug_edible, DropType},
		targets::{Bowl, ClearOnDayTransition}, PachinkoSystemSet,
		pegs::{Peg, PEG_DEPTH, PegType, PegType::ItemPeg, PegType::PachinkoPeg},
		states::{DayState, PauseState, GameState}
	}, setup::UserData
};

use super::parlor::{DrugPeg, HORIZONTAL_SPEED, VERTICAL_SPEED};

pub struct FoodMinigame;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct FoodSystemSet;

#[derive(Component)]
pub struct Soup;

#[derive(Component)]
pub struct AddictionPeg(pub bool);

#[derive(Component)]
pub struct HardcoreAddictionPeg(pub bool);

impl Plugin for FoodMinigame {
	fn build(&self, app: &mut App) {
		app
		.configure_set(FoodSystemSet
			.run_if(in_state(DayState::Evening))
			.run_if(in_state(PauseState::Unpaused))
			.run_if(in_state(GameState::Game)))
		.add_system(eat_food
			.run_if(on_event::<BallCollisionEvent>())
			.in_set(FoodSystemSet)
			.after(PachinkoSystemSet))
		.add_system(increase_addiction
			.run_if(on_event::<BallCollisionEvent>())
			.in_set(FoodSystemSet)
			.after(PachinkoSystemSet))
		.add_system(handle_mouth_soup
			.run_if(on_event::<BallTargetHit>())
			.in_set(FoodSystemSet)
			.in_set(PachinkoSystemSet))
		.add_system(move_targets
			.in_set(FoodSystemSet)
			.before(PachinkoSystemSet))
		.add_system(move_addiction_pegs
			.in_set(FoodSystemSet)
			.before(PachinkoSystemSet))
		.add_system(move_hardcore_addiction_pegs
			.in_set(FoodSystemSet)
			.before(PachinkoSystemSet))
		.add_system(spawn_food_pegs.in_schedule(OnEnter(DayState::Evening)))
		;
	}
}

fn eat_food(
	mut commands: Commands,
	mut collision_events: EventReader<BallCollisionEvent>,
	mut user_data: ResMut<UserData>,
) {
	let mut peg_to_despawn = Vec::new();
	for collision in collision_events.iter() {
		match collision.collision_type {
			BallCollisionEventType::Ball => (),
			BallCollisionEventType::Peg{peg_type: ItemPeg(drop_type), peg, ..} => {
				match drop_type.get_type() {
					DropCategory::Drug => {
						user_data.royal = (user_data.royal + 5.0).clamp(0.0, 100.0);
						user_data.stress = (user_data.stress + 5.0).clamp(0.0, 100.0);
						peg_to_despawn.push(peg);
					},
					_ => {
						if drop_type.is_edible(){
							user_data.stress = (user_data.stress - 1.0).clamp(0.0, 100.0);
							peg_to_despawn.push(peg);
						} else {
							user_data.stress = (user_data.stress + 5.0).clamp(0.0, 100.0);
						}
					},
				}},
			BallCollisionEventType::Peg{peg_type: PachinkoPeg, ..} => (),
		}
	}
	//println!("{:?}", peg_to_despawn);
	peg_to_despawn.sort();
	peg_to_despawn.dedup();
	for peg in peg_to_despawn {
		//println!("{:?}", peg);
		commands.entity(peg).despawn_recursive();
	}	
}

fn handle_mouth_soup(
	mut collision_events: EventReader<BallTargetHit>,
	mut user_data: ResMut<UserData>,
	soup_query: Query<&Transform, (With<Soup>, Without<Bowl>)>,
) {
	for ball_event in collision_events.iter() {
		let target_entity = ball_event.target;
		if let Ok(_) = soup_query.get(target_entity) {
			user_data.stress = (user_data.stress - 10.0).clamp(0.0, 100.0);
		}
	}
}


fn move_targets(
	mut soup_query: Query<(&Transform, &mut Velocity), With<Soup>>,
	time: Res<Time>,
) {
	for (transform, mut velocity) in soup_query.iter_mut() {
		let target = transform.translation.x + velocity.0.x * time.delta_seconds();
		if target > 850.0 && velocity.0.x > 0.0 || target < 150.0 && velocity.0.x < 0.0 {
			velocity.0.x = -velocity.0.x;
		}
	}
}

fn spawn_food_pegs(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Edible Pegs
	let mut map: Vec<(f32, f32)> = vec![];
	// Layered Walls
	for i in 0..19 {
		for j in 0..13 {
			if !((i == 8 || i == 9 || i == 10) && (j == 5 || j == 6 || j == 7)) && !(i == 11 && j == 6){
				map.push((i as f32*48.0 + 56.0 + (j as f32 % 2.0)*24.0, j as f32*48.0 + 48.0));
			}
		}
	}

	for loc in map {
		let peg = random_non_drug_edible();
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
			)
		);
	}

	// Drug Peg
	let peg = DropType::Pill;
	let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(512.0, 336.0, PEG_DEPTH)
					.with_scale(Vec3::new(1.25, 1.25, PEG_DEPTH)),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
			)
		);

	// Addiction Pegs
	// Weed weed
	let map = vec![
		(446.0,148.0),(473.0,170.0),(500.0,191.0),(526.0,172.0),(550.0,150.0),(498.0,222.0),(500.0,257.0),
		(502.0,283.0),(502.0,320.0),(502.0,354.0),(502.0,387.0),(502.0,426.0),(502.0,455.0),(507.0,490.0),
		(507.0,528.0),(478.0,503.0),(475.0,467.0),(475.0,434.0),(477.0,402.0),(474.0,372.0),(528.0,501.0),
		(530.0,467.0),(529.0,439.0),(528.0,412.0),(530.0,387.0),(555.0,397.0),(585.0,419.0),(606.0,440.0),
		(624.0,460.0),(654.0,479.0),(645.0,445.0),(626.0,422.0),(609.0,396.0),(582.0,381.0),(557.0,365.0),
		(532.0,349.0),(450.0,378.0),(428.0,399.0),(405.0,426.0),(388.0,454.0),(358.0,461.0),(364.0,435.0),
		(378.0,408.0),(403.0,380.0),(426.0,359.0),(464.0,344.0),(440.0,331.0),(406.0,330.0),(378.0,331.0),
		(344.0,339.0),(465.0,317.0),(446.0,294.0),(426.0,274.0),(394.0,261.0),(524.0,324.0),(552.0,321.0),
		(583.0,322.0),(612.0,329.0),(639.0,341.0),(527.0,297.0),(546.0,283.0),(565.0,274.0),(592.0,270.0),
		(475.0,291.0),(479.0,263.0),(478.0,240.0),(475.0,220.0),(522.0,270.0),(519.0,241.0),(522.0,213.0),
		(476.0,202.0),(454.0,173.0),(541.0,171.0),(536.0,188.0),];
	//for i in 0..19 {
	//	for j in 0..12 {
	//		map.push((i as f32*48.0 + 56.0 + (j as f32 % 2.0)*24.0 - 1000.0, j as f32*48.0 + 48.0));
	//	}
	//}
	

	for loc in map {
		let peg = DropType::Weed;
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0 - 700.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
				AddictionPeg(true)
			)
		);
	}

	// Hardcore Addiction Pegs
	// Mushroom Cap
	let map = vec![
		(324.0,342.0),(355.0,335.0),(378.0,334.0),(399.0,331.0),(422.0,330.0),(457.0,329.0),(473.0,329.0),
		(439.0,329.0),(497.0,328.0),(527.0,330.0),(556.0,330.0),(571.0,335.0),(596.0,339.0),(624.0,351.0),
		(599.0,362.0),(588.0,386.0),(591.0,406.0),(609.0,434.0),(632.0,450.0),(621.0,471.0),(602.0,447.0),
		(581.0,434.0),(556.0,419.0),(531.0,429.0),(511.0,445.0),(506.0,469.0),(508.0,489.0),(520.0,507.0),
		(542.0,520.0),(568.0,527.0),(588.0,523.0),(610.0,498.0),(596.0,516.0),(561.0,548.0),(550.0,553.0),
		(527.0,569.0),(535.0,541.0),(506.0,548.0),(507.0,572.0),(487.0,555.0),(479.0,548.0),(462.0,536.0),
		(441.0,529.0),(426.0,527.0),(404.0,527.0),(390.0,528.0),(376.0,533.0),(360.0,542.0),(354.0,542.0),
		(347.0,529.0),(338.0,517.0),(323.0,495.0),(314.0,479.0),(309.0,468.0),(304.0,458.0),(295.0,435.0),
		(283.0,411.0),(282.0,397.0),(275.0,368.0),(281.0,355.0),(302.0,348.0),(302.0,419.0),(312.0,405.0),
		(330.0,398.0),(339.0,387.0),(356.0,365.0),(359.0,355.0),(325.0,438.0),(342.0,423.0),(363.0,404.0),
		(379.0,378.0),(393.0,364.0),(422.0,365.0),(451.0,360.0),(450.0,387.0),(417.0,418.0),(386.0,411.0),
		(445.0,414.0),(367.0,510.0),(391.0,495.0),(403.0,480.0),(404.0,454.0),(372.0,446.0),(346.0,461.0),
		(340.0,486.0),(370.0,431.0),(401.0,435.0),(433.0,443.0),(427.0,467.0),(422.0,493.0),(455.0,500.0),
		(477.0,475.0),(462.0,440.0),(479.0,512.0),(499.0,518.0),(482.0,439.0),(482.0,406.0),(474.0,378.0),
		(504.0,371.0),(513.0,406.0),(570.0,362.0),(550.0,346.0),(519.0,356.0),];
	//for i in 0..19 {
	//	for j in 0..12 {
	//		map.push((i as f32*48.0 + 56.0 + (j as f32 % 2.0)*24.0, j as f32*48.0 + 48.0 - 800.0));
	//	}
	//}
	

	for loc in map {
		let peg = DropType::Mushroom;
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1- 700.0, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
				HardcoreAddictionPeg(true)
			)
		);
	}

	// Hardcore Addiction Pegs
	// Mushroom Stalk
	let map = vec![
		(410.0,310.0),(407.0,284.0),(407.0,261.0),(399.0,230.0),(386.0,205.0),(371.0,178.0),(362.0,159.0),
		(388.0,155.0),(423.0,155.0),(457.0,156.0),(480.0,155.0),(512.0,159.0),(546.0,161.0),(575.0,161.0),
		(568.0,188.0),(552.0,210.0),(535.0,240.0),(528.0,270.0),(526.0,309.0),(487.0,307.0),(453.0,310.0),
		(432.0,285.0),(463.0,283.0),(492.0,283.0),(496.0,258.0),(461.0,257.0),(428.0,257.0),(427.0,228.0),
		(464.0,227.0),(495.0,229.0),(520.0,201.0),(488.0,199.0),(456.0,198.0),(428.0,198.0),(401.0,179.0),
		(407.0,202.0),(443.0,180.0),(482.0,179.0),(520.0,181.0),(539.0,187.0),(522.0,230.0),];
	//for i in 0..19 {
	//	for j in 0..12 {
	//		map.push((i as f32*48.0 + 56.0 + (j as f32 % 2.0)*24.0, j as f32*48.0 + 48.0 - 800.0));
	//	}
	//}
	

	for loc in map {
		let peg = DropType::Shot;
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1 - 700.0, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
				HardcoreAddictionPeg(true)
			)
		);
	}

	// Hardcore Addiction Pegs
	// Mushroom Spots
	let map = vec![
		(378.0,561.0),(403.0,549.0),(428.0,548.0),(451.0,555.0),(470.0,567.0),(487.0,579.0),(467.0,583.0),
		(453.0,582.0),(429.0,577.0),(405.0,577.0),(390.0,569.0),(554.0,504.0),(545.0,494.0),(539.0,479.0),
		(546.0,459.0),(567.0,462.0),(591.0,472.0),(579.0,495.0),(564.0,480.0),(308.0,390.0),(328.0,382.0),
		(334.0,368.0),(302.0,372.0),(451.0,467.0),(531.0,393.0),(526.0,364.0),(547.0,371.0),(552.0,395.0),
		(636.0,371.0),(638.0,400.0),(639.0,429.0),(621.0,412.0),(618.0,385.0),(648.0,384.0),(646.0,410.0),
		(643.0,361.0),(408.0,390.0),(430.0,356.0),(384.0,463.0),(379.0,485.0),(354.0,484.0),(357.0,464.0),
		(441.0,486.0),(487.0,356.0),(466.0,407.0),];
	//for i in 0..19 {
	//	for j in 0..12 {
	//		map.push((i as f32*48.0 + 56.0 + (j as f32 % 2.0)*24.0, j as f32*48.0 + 48.0 - 800.0));
	//	}
	//}
	

	for loc in map {
		let peg = DropType::Pill;
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1 - 700.0, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
				HardcoreAddictionPeg(true)
			)
		);
	}

	// Inedible Pegs
	let mut map: Vec<(f32, f32)> = vec![];
	// Left Wall
	for i in 0..13 {
		map.push((16.0, i as f32*48.0 + 48.0));
	}
	// Right Wall
	for i in 0..13 {
		map.push((984.0, i as f32*48.0 + 48.0));
	}

	for loc in map {
		let peg = random_inedible_except_special();
		let peg_texture_handle = asset_server.load(peg.get_path());
		commands.spawn(
			(
				SpriteBundle {
					transform: Transform::from_xyz(loc.0, loc.1, PEG_DEPTH),
					texture: peg_texture_handle.clone(),
					..default()
				},
				Peg(PegType::ItemPeg(peg)),
				ClearOnDayTransition,
				DrugPeg(loc.1, true)
			)
		);
	}
}

fn increase_addiction(
	mut collision_events: EventReader<BallCollisionEvent>,
	mut user_data: ResMut<UserData>,
) {
	for collision in collision_events.iter() {
		match collision.collision_type {
			BallCollisionEventType::Ball => (),
			BallCollisionEventType::Peg{peg_type: ItemPeg(drop_type), ..} => {
				if drop_type == DropType::Pill 
				|| drop_type == DropType::Mushroom 
				|| drop_type == DropType::Shot 
				|| drop_type == DropType::Weed {
					user_data.drugs_taken += 1.0;
				}
			},
			BallCollisionEventType::Peg{..} => (),
		}
	}
}

fn move_addiction_pegs(
	mut addiction_peg_query: Query<(&mut Transform, &mut AddictionPeg)>,
	user_data: Res<UserData>,
	time: Res<Time>,
) {
	if user_data.drugs_taken >= 1.0 {
		for (mut transform, mut addiction_peg) in addiction_peg_query.iter_mut() {
			if addiction_peg.0 {transform.translation.x += HORIZONTAL_SPEED * 1.5 * time.delta_seconds()
				} else {transform.translation.x += -HORIZONTAL_SPEED * 1.5 * time.delta_seconds()};
			if transform.translation.x >= 984.0 {addiction_peg.0 = false};
			if transform.translation.x <= 16.0 {addiction_peg.0 = true};
		}
	}
}

fn move_hardcore_addiction_pegs(
	mut hardcore_addiction_peg_query: Query<(&mut Transform, &mut HardcoreAddictionPeg)>,
	user_data: Res<UserData>,
	time: Res<Time>,
) {
	if user_data.drugs_taken >= 30.0 {
		for (mut transform, mut hardcore_addiction_peg) in hardcore_addiction_peg_query.iter_mut() {
			if hardcore_addiction_peg.0 {transform.translation.y += VERTICAL_SPEED * 2.5 * time.delta_seconds()
				} else {transform.translation.y += -VERTICAL_SPEED * 2.5 * time.delta_seconds()};
			if transform.translation.y >= 624.0 {hardcore_addiction_peg.0 = false};
			if transform.translation.x <= 16.0 {hardcore_addiction_peg.0 = true};
		}
	}
}