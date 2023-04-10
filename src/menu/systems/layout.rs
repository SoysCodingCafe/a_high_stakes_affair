use bevy::prelude::*;
use bevy::text::BreakLineOn;

use super::super::styles::*;
use super::super::components::*;

pub fn spawn_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	//let menu_entity = build_menu(&mut commands, &asset_server);
	build_menu(&mut commands, &asset_server);
}

pub fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	build_ui(&mut commands, &asset_server);
}

pub fn build_menu(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>
) -> Entity {
	commands
		// Menu Node Bundle 
		.spawn(
			(NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
					gap: Size::new(Val::Px(16.0), Val::Px(16.0)),
					..default()
				},
				//background_color: Color::GREEN.into(),
				..default()
			},
			Menu,
		))
		.with_children(|parent| {
			// Title
			parent.spawn(
				TextBundle {
					text: Text {
						sections: vec![
							TextSection::new(
								"A",
								get_title_text_style(asset_server),
							)
						],
						alignment: TextAlignment::Center,
						..default()
					},
					..default()
			});
			// New Game Button
			parent.spawn(
				(
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: IDLE_BUTTON_COLOR.into(),
						..default()
					},
					NewGameButton,
				))
				.with_children(|parent| {
					// New Game Button Text
					parent.spawn(
						TextBundle {
							text: Text {
								sections: vec![
									TextSection::new(
										"New Game",
										get_button_text_style(asset_server),
									)
								],
								alignment: TextAlignment::Center,
								..default()
							},
							..default()
					});
				})
			;

			// Load Game Button
			parent.spawn(
				(
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: IDLE_BUTTON_COLOR.into(),
						..default()
					},
					LoadGameButton,
				))
				.with_children(|parent| {
					// Load Game Button Text
					parent.spawn(
						TextBundle {
							text: Text {
								sections: vec![
									TextSection::new(
										"Load Game",
										get_button_text_style(asset_server),
									)
								],
								alignment: TextAlignment::Center,
								..default()
							},
							..default()
					});
				})
			;

			// Quit Game Button
			parent.spawn(
				(
					ButtonBundle {
						style: BUTTON_STYLE,
						background_color: IDLE_BUTTON_COLOR.into(),
						..default()
					},
					QuitGameButton,
				))
				.with_children(|parent| {
					// Quit Game Button Text
					parent.spawn(
						TextBundle {
							text: Text {
								sections: vec![
									TextSection::new(
										"Quit Game",
										get_button_text_style(asset_server),
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
		.id()
}

pub fn build_ui(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>
) -> Entity {
	commands
		// UI Node Bundle
		.spawn(
		(NodeBundle {
			style: Style {
				flex_direction: FlexDirection::Row,
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				..default()
			},
			background_color: Color::Rgba {red: 0.1, green: 0.8, blue: 0.1, alpha: 0.0}.into(),
			..default()
		},
		Ui,
	))
	.with_children(|parent| {
		// Machine
		parent.spawn(
			NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::FlexStart,
					align_items: AlignItems::Center,
					size: Size::new(Val::Px(1000.0), Val::Px(900.0)),
					..default()
				},
				background_color: Color::Rgba {red: 0.8, green: 0.1, blue: 0.1, alpha: 0.0}.into(),
				..default()
		})
		.with_children(|parent| {
			// Status Bar
			parent.spawn(
				NodeBundle {
					style: Style {
						flex_direction: FlexDirection::Column,
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						size: Size::new(Val::Px(1000.0), Val::Px(100.0)),
						..default()
					},
					background_color: Color::Rgba {red: 0.1, green: 0.1, blue: 0.8, alpha: 0.0}.into(),
					..default()
				});
		})
		;
		// VN
		parent.spawn(
			NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::FlexEnd,
					align_items: AlignItems::Center,
					size: Size::new(Val::Px(600.0), Val::Px(900.0)),
					..default()
				},
				..default()
		})
		.with_children(|parent| {
			// Text Box Node Bundle
			parent.spawn(
				NodeBundle {
					style: Style {
						justify_content: JustifyContent::FlexStart,
						align_items: AlignItems::FlexStart,
						size: Size::new(Val::Px(600.0), Val::Px(400.0)),
						..default()
					},
					background_color: Color::Rgba {red: 0.1, green: 0.1, blue: 0.1, alpha: 0.0}.into(),
					..default()
			})
			.with_children(|parent| {
				// Text Box
				parent.spawn((
					TextBundle {
						style: Style {
							margin: UiRect {
								left: Val::Px(48.0),
								//right: Val::Px(200.0),
								top: Val::Px(48.0),
								//bottom: Val::Px(-96.0)
								..default()
							},
							max_size: Size::new(Val::Px(500.0), Val::Px(320.0)),
							..default()
						},
						text: Text {
							sections: vec![
								TextSection::new(
									"",
									get_vn_text_style(asset_server),
								)
							],
							alignment: TextAlignment::Left,
							linebreak_behaviour: BreakLineOn::WordBoundary,
						},
						..default()
					},
					DialogueText,
				));
			});
		});
	})
	.id()
}