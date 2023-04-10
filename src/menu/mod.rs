use bevy::prelude::*;

use crate::{despawn_entities_with, game::states::{GameState, PauseState, DayState}};

use self::{systems::{layout::{spawn_menu, spawn_ui}, interactions::{new_game_button_interaction, load_game_button_interaction, quit_game_button_interaction}}, components::{Menu, Ui}, dialogue::update_dialogue};

mod dialogue;
mod systems;
mod styles;
pub mod components;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_system(update_dialogue.run_if(not(in_state(DayState::Dawn))))
		.add_system(spawn_menu.in_schedule(OnEnter(GameState::Menu)))
		.add_system(spawn_ui.in_schedule(OnEnter(PauseState::Unpaused)))
		.add_system(despawn_entities_with::<Menu>.in_schedule(OnExit(GameState::Menu)))
		.add_system(despawn_entities_with::<Ui>.in_schedule(OnExit(GameState::Game)))
		.add_system(despawn_entities_with::<Ui>.in_schedule(OnExit(PauseState::Unpaused)))
		.add_systems((
				new_game_button_interaction,
				load_game_button_interaction,
				quit_game_button_interaction,
		))
		;
    }
}