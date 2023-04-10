use bevy::prelude::*;

// Modules
mod backgrounds;
mod balls;
pub mod pegs;
pub mod states;
pub mod food;
pub mod physics;
#[cfg(debug_assertions)]
mod debug;
pub mod targets;
mod minigames;

// Plugins
use self::backgrounds::BackgroundsPlugin;
use self::balls::BallsPlugin;
use self::states::StatesPlugin;
use self::physics::PhysicsPlugin;
#[cfg(debug_assertions)]
use self::debug::GameDebugPlugin;
use self::targets::TargetsPlugin;
use self::minigames::{FoodMinigame, WorkMinigame, ParlorMinigame};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
		.configure_set(
			PachinkoSystemSet
			.run_if(not(in_state(states::DayState::Dawn)))
			.run_if(in_state(states::GameState::Game))
			.run_if(in_state(states::PauseState::Unpaused))
		)
		.add_plugin(BackgroundsPlugin)
		.add_plugin(BallsPlugin)
		.add_plugin(PhysicsPlugin)
		.add_plugin(StatesPlugin)
		.add_plugin(TargetsPlugin)
		// Individual minigames
		.add_plugin(FoodMinigame)
		.add_plugin(WorkMinigame)
		.add_plugin(ParlorMinigame)
		;
		#[cfg(debug_assertions)]
		app.add_plugin(GameDebugPlugin);
	}
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
struct PachinkoSystemSet;
