use bevy::{prelude::*, render::view::RenderLayers};

use crate::{game::{states::{PauseState, DayState}, targets::ClearOnDayTransition}, setup::{despawn_entities_with, UserData}, menu::components::QuitGameButton};

pub const IDLE_BUTTON_COLOR: Color = Color::BLUE;
pub const HOVERED_BUTTON_COLOR: Color = Color::PINK;
pub const CLICKED_BUTTON_COLOR: Color = Color::ORANGE;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_system(confirm_button_interaction.run_if(in_state(PauseState::Paused)))
		.add_system(spawn_popup.in_schedule(OnEnter(PauseState::Paused)))
		.add_system(despawn_entities_with::<Popup>.in_schedule(OnExit(PauseState::Paused)))
		.add_system(despawn_entities_with::<Tutorial>.in_schedule(OnExit(PauseState::Paused)))
		;
	}
}

#[derive(Component)]
struct Popup;

#[derive(Component)]
struct Tutorial;

#[derive(Component)]
pub struct ConfirmButton;

pub fn confirm_button_interaction(
	mut button_query: Query<
	(&Interaction, &mut BackgroundColor), 
	(Changed<Interaction>, With<ConfirmButton>)>,
	current_day_state: Res<State<DayState>>,
	mut next_day_state: ResMut<NextState<DayState>>,
	mut next_pause_state: ResMut<NextState<PauseState>>,
	mut user_data: ResMut<UserData>,
) {
	if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
		match *interaction {
			Interaction::Clicked => {
				*background_color = CLICKED_BUTTON_COLOR.into();
				if current_day_state.0 != user_data.time {
					next_day_state.set(user_data.time);
				};
				next_pause_state.set(PauseState::Unpaused);
				user_data.lvl_init = true;
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

pub fn spawn_popup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	user_data: Res<UserData>,
) {
	let time = match user_data.time {
		DayState::Dawn => String::from("Error"),
		DayState::Morning => String::from("Work - Morning"),
		DayState::Evening => String::from("Dinner - Evening"),
		DayState::Night => String::from("Arcade - Night"),
	};
	let popup_text = if user_data.lvl_init {
		String::from("Paused")
	} else {
		String::from(format!("{} - Day {}", time, user_data.day))
	};

	let button_text = if user_data.lvl_init {
		String::from("Resume")
	} else {

		String::from("Begin")
	};
	match user_data.time {
		DayState::Dawn => {	},
		DayState::Morning => {
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/work.png"),
						..default()
					},
					Tutorial,
				)
			);
		},
		DayState::Evening => {
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/food.png"),
						..default()
					},
					Tutorial,
				)
			);
		},
		DayState::Night => {
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
			commands.spawn(
				(
					SpriteBundle {
						transform: Transform::from_xyz(800.0, 450.0, -0.01),
						texture: asset_server.load("backgrounds/parlor.png"),
						..default()
					},
					Tutorial,
				)
			);
		},
	}
	let popup = build_popup(&mut commands, &asset_server, popup_text, button_text);
	commands.entity(popup).insert(RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8));
}

fn build_popup(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	popup_text: String,
	button_text: String,
) -> Entity {
commands
		// Popup Node Bundle
		.spawn(
			(NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
					gap: Size::new(Val::Px(32.0), Val::Px(32.0)),
					..default()
				},
				background_color: Color::Rgba{red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0}.into(),
				..default()
			},
			Popup,
		))
		.with_children(|parent| {
			// Popup Text
			parent.spawn(
				TextBundle {
					text: Text {
						sections: vec![
							TextSection::new(
								popup_text,
								TextStyle {
									font: asset_server.load("fonts/FiraSans-Bold.ttf"),
									font_size: 64.0,
									color: Color::PINK,
								},
							)
						],
						alignment: TextAlignment::Center,
						..default()
					},
					..default()
			});
			// Buttons Node Bundle
			parent.spawn(
				(
					NodeBundle {
						style: Style {
							flex_direction: FlexDirection::Row,
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							size: Size::new(Val::Percent(30.0), Val::Percent(30.0)),
							gap: Size::new(Val::Px(16.0), Val::Px(16.0)),
							..default()
						},
						..default()
					},
				))
				.with_children(|parent| {
					// Confirm Button
					parent.spawn(
						(
							ButtonBundle {
								style: Style {
									justify_content: JustifyContent::Center,
									align_items: AlignItems::Center,
									size: Size::new(Val::Px(256.0), Val::Px(64.0)),
									..default()
								},
								background_color: IDLE_BUTTON_COLOR.into(),
								..default()
							},
							ConfirmButton,
						))
						.with_children(|parent| {
							// Confirm Button Text
							parent.spawn(
								TextBundle {
									text: Text {
										sections: vec![
											TextSection::new(
												button_text,
												TextStyle {
													font: asset_server.load("fonts/FiraSans-Bold.ttf"),
													font_size: 32.0,
													color: Color::CYAN,
												},
											)
										],
										alignment: TextAlignment::Center,
										..default()
									},
									..default()
							});
						})
					;
					// Quit Button
					parent.spawn(
						(
							ButtonBundle {
								style: Style {
									justify_content: JustifyContent::Center,
									align_items: AlignItems::Center,
									size: Size::new(Val::Px(256.0), Val::Px(64.0)),
									..default()
								},
								background_color: IDLE_BUTTON_COLOR.into(),
								..default()
							},
							QuitGameButton,
						))
						.with_children(|parent| {
							// Quit Button Text
							parent.spawn(
								TextBundle {
									text: Text {
										sections: vec![
											TextSection::new(
												"Quit",
												TextStyle {
													font: asset_server.load("fonts/FiraSans-Bold.ttf"),
													font_size: 32.0,
													color: Color::CYAN,
												},
											)
										],
										alignment: TextAlignment::Center,
										..default()
									},
									..default()
							});
						})
					;
				})
			;
		})
	.id()
}