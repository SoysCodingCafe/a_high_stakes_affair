use bevy::prelude::*;

use crate::{menu::{components::DialogueText, styles::get_vn_text_style}, setup::UserData, game::states::{DayState, DayTimer}};

pub fn update_dialogue(
	mut dialogue_query: Query<&mut Text, With<DialogueText>>,
	asset_server: Res<AssetServer>,
	user_data: Res<UserData>,
	day_timer: Res<DayTimer>,
) {
	let dialogue_line = if day_timer.0.percent() < 0.75 { 
		 next_line(&user_data)
	} else {"Uh oh, looks like it's almost time for you to go!".to_string()};
	for mut text in dialogue_query.iter_mut() {
		text.sections = vec![
			TextSection::new(
				&dialogue_line,
				get_vn_text_style(&asset_server),
			)
		];
	}
}

pub fn next_line(
	user_info: &UserData,
) -> String {
	match user_info.time {
		DayState::Dawn => "How did you even get here?".to_string(),
		DayState::Morning => 
			if user_info.money > 50.0 {"You know, sometimes I think you should be MY boss!".to_string()}
			else {"Pack those baskets rookie! Make sure you don't mess up the orders or it'll be stressful for the customers... and you!".to_string()},
		DayState::Evening => 
			if user_info.royal > 50.0 {"Now you're starting to see things my way! hahaHAHAHAHAHA".to_string()}
			else {"Hey, nice soup you got there. Care to share? I've got a few ingredients of my own I could add...".to_string()},
		DayState::Night => 
			if (user_info.day % 2.0) == 1.0 {
				if user_info.flirt > 50.0 {"Hey, why don't we get out of this place? I know a nice coffee shop nearby!".to_string()}
				else {"Ah, so you met gramps? He's got some crazy theories. You'll probably see him again if you come back tomorrow.".to_string()}
			} else {
				if user_info.stress > 50.0 {"Wow, you're looking pretty burnt out. Trust me, I know how it feels. Come on, let's take a break and go for a walk, it'll help us freshen up.".to_string()}
				else {"Ah, a new face! You looking for tips from the master? You wanna hit all the bells to trigger the jackpot, but you'd have to be HIGH to even believe it's possible.".to_string()}
					//"Ah, I remember being like you back in the day, bright eyed and ready to go. The world will beat you down soon enough. I hear if you hit all three bells you win some prize, but those pegs are blocking the way. If only I could get a different perspective on things...".to_string()}
			}
	}
}