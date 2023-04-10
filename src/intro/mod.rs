use bevy::prelude::*;

// Modules
// mod slideshow;
pub mod tutorial;
mod warning;
mod cutscene;

// Plugins
use tutorial::TutorialPlugin;
use warning::WarningPlugin;
use cutscene::CutscenePlugin;

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_plugin(WarningPlugin)
		//.add_plugin(SlideshowPlugin)
		.add_plugin(TutorialPlugin)
		.add_plugin(CutscenePlugin)
		;
	}
}
