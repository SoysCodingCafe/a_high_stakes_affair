use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::game::states::{DayState, PauseState};

use super::super::super::setup::{SaveData, UserData};
use super::super::GameState;
use super::super::components::*;
use super::super::styles::{IDLE_BUTTON_COLOR, HOVERED_BUTTON_COLOR, CLICKED_BUTTON_COLOR, DISABLED_BUTTON_COLOR};

pub fn new_game_button_interaction(
	mut button_query: Query<
	(&Interaction, &mut BackgroundColor), 
	(Changed<Interaction>, With<NewGameButton>)>,
	mut next_state: ResMut<NextState<GameState>>,
) {
	if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
		match *interaction {
			Interaction::Clicked => {
				*background_color = CLICKED_BUTTON_COLOR.into();
				next_state.set(GameState::Intro);
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = IDLE_BUTTON_COLOR.into();
			}
		}
	}
}

pub fn load_game_button_interaction(
	mut button_query: Query<
	(&Interaction, &mut BackgroundColor), 
	(Changed<Interaction>, With<LoadGameButton>)>,
	mut next_game_state: ResMut<NextState<GameState>>,
	mut next_day_state: ResMut<NextState<DayState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	pkv: Res<PkvStore>,
	mut user_data: ResMut<UserData>,
) {
	if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
		if let Ok(save_data) = pkv.get::<SaveData>("user_info") {
			match *interaction {
				Interaction::Clicked => {
					*background_color = CLICKED_BUTTON_COLOR.into();

					user_data.name = save_data.name;
					user_data.money = save_data.money;
					user_data.stress = save_data.stress;
					user_data.flirt = save_data.flirt;
					user_data.royal = save_data.royal;
					user_data.day = save_data.day;
					user_data.time = save_data.time;
					user_data.lvl_init = false;

					match user_data.time {
						DayState::Dawn => (),
						DayState::Morning => next_day_state.set(DayState::Morning),
						DayState::Evening => next_day_state.set(DayState::Evening),
						DayState::Night => next_day_state.set(DayState::Night),
					}
					next_game_state.set(GameState::Game);
					next_pause_state.set(PauseState::Paused);
				}
				Interaction::Hovered => {
					*background_color = HOVERED_BUTTON_COLOR.into();
				}
				Interaction::None => {
					*background_color = IDLE_BUTTON_COLOR.into();
				}
			}
		} else {
			*background_color = DISABLED_BUTTON_COLOR.into();
		}
	}
}

pub fn quit_game_button_interaction(
	mut button_query: Query<
	(&Interaction, &mut BackgroundColor), 
	(Changed<Interaction>, With<QuitGameButton>)>,
	mut exit_app: EventWriter<AppExit>,
) {
	if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
		match *interaction {
			Interaction::Clicked => {
				*background_color = CLICKED_BUTTON_COLOR.into();
				exit_app.send(AppExit);
			}
			Interaction::Hovered => {
				*background_color = HOVERED_BUTTON_COLOR.into();
			}
			Interaction::None => {
				*background_color = IDLE_BUTTON_COLOR.into();
			}
		}
	}
}