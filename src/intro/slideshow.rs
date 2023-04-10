use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{game::states::{GameState, PauseState, DayState}, setup::{SaveData, UserData}};

pub struct SlideshowPlugin;

impl Plugin for SlideshowPlugin {
    fn build(&self, app: &mut App) {
        app
		//.add_system(start_slideshow.in_schedule(OnEnter(GameState::Intro)))
		//.add_system(advance_slideshow.run_if(in_state(GameState::Intro)).after(start_slideshow))
		//.add_system(display_slideshow.run_if(in_state(GameState::Intro)).after(start_slideshow))
		;
	}
}

#[derive(Resource)]
struct CurrentSlide(u8);

#[derive(Component)]
struct Slideshow;

fn start_slideshow(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(CurrentSlide(0));
	commands.spawn(
		(
			SpriteBundle {
				texture: asset_server.load("intro/intro0.png"),
				transform: Transform::from_xyz(800.0, 450.0, 0.0),
				..default()
			},
			Slideshow,
		)
	);
}

fn advance_slideshow(
	mut current_slide: ResMut<CurrentSlide>,
	mouse: Res<Input<MouseButton>>,
) {
	if mouse.just_pressed(MouseButton::Left) {
		current_slide.0 += 1;
	}
}

fn display_slideshow(
	mut commands: Commands,
	mut slideshow_query: Query<(Entity, With<Slideshow>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut pkv: ResMut<PkvStore>,
	mut user_data: ResMut<UserData>,
	current_slide: Res<CurrentSlide>,
	asset_server: Res<AssetServer>,
) {
	if current_slide.is_changed() {
		let (entity, _) = slideshow_query.get_single_mut().unwrap();
		commands.entity(entity).despawn_recursive();
		if current_slide.0 < 3 {
			commands.spawn(
				(
					SpriteBundle {
						texture: asset_server.load(format!("intro/intro{}.png", current_slide.0)),
						transform: Transform::from_xyz(800.0, 450.0, 0.0),
						..default()
					},
					Slideshow,
				)
			);
		} else {
			let save_data = SaveData {
				name: "BevyEnjoyer123".to_string(),
				money: 30.0,
				stress: 0.0,
				flirt: 50.0,
				royal: 0.0,
				drugs_taken: 0.0,
				day: 0.0,
				time: DayState::Night,
				lvl_init: false,
			};
			pkv.set("user_info", &save_data)
				.expect("Unable to store user");

			user_data.name = save_data.name;
			user_data.money = save_data.money;
			user_data.stress = save_data.stress;
			user_data.flirt = save_data.flirt;
			user_data.royal = save_data.royal;
			user_data.day = save_data.day;
			user_data.time = save_data.time;
			user_data.lvl_init = false;

			next_game_state.set(GameState::Game);
			next_pause_state.set(PauseState::Paused);
		}
	}
}