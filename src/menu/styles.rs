use bevy::prelude::*;

pub const IDLE_BUTTON_COLOR: Color = Color::BLUE;
pub const HOVERED_BUTTON_COLOR: Color = Color::PINK;
pub const CLICKED_BUTTON_COLOR: Color = Color::ORANGE;
pub const DISABLED_BUTTON_COLOR: Color = Color::DARK_GRAY;

pub const BUTTON_STYLE: Style = Style {
	justify_content: JustifyContent::Center,
	align_items: AlignItems::Center,
	size: Size::new(Val::Px(256.0), Val::Px(64.0)),
	..Style::DEFAULT
};

pub fn get_title_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 100.0,
		color: Color::rgba(0.0, 0.0, 0.0, 0.0),
	}
}

pub fn get_button_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 32.0,
		color: Color::CYAN,
	}
}

pub fn get_vn_text_style(
	asset_server: &Res<AssetServer>
) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/FiraSans-Bold.ttf"),
		font_size: 32.0,
		color: Color::CYAN,
	}
}