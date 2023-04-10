use bevy::{prelude::*, window::PrimaryWindow};
use serde::{Serialize, Deserialize};

use crate::vfx::RenderTargetImage;
use crate::game::states::DayState;

pub fn spawn_camera(
	mut commands: Commands,
	window_query: Query<&Window, With<PrimaryWindow>>,
	target_image: Res<RenderTargetImage>,
) {
	let window = window_query.get_single().expect("No/Multiple windows, one expected");
	//let target_image = target_image.get_single().expect("No/Multiple RenderTargetImage(s), make sure to first spawn the RenderTargetImage via the vfxplugin.");

	// Actual render camera, renders to a texture
	commands.spawn((
		Camera2dBundle {
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			camera: Camera {
				// This causes UI to not work
				target: bevy::render::camera::RenderTarget::Image(target_image.image.clone()),
				..default()
			},
			..default()
		},
		UiCameraConfig { show_ui: true }
	));
	// Camera that renders before the real camera, so it shouldn't actually display anything
	// Used to render UI so UI works.
	commands.spawn((
		Camera2dBundle {
			transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
			camera: Camera {
				order: -1, // order -1 so it renders before the VFX camera
				..default()
			},
			..default()
		},
		UiCameraConfig { show_ui: true }
	));
}

pub fn despawn_entities_with<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
	for entity in &to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}

#[derive(Resource)]
pub struct UserData {
	pub name: String,
	pub money: f32,
	pub stress: f32,
	pub flirt: f32,
	pub royal: f32,
	// Used for VFX, can take on higher values than normal royal, but decreases quickly with time
	pub unstable_royal: f32,
	pub drugs_taken: f32,
	pub day: f32,
	pub time: DayState,
	pub lvl_init: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SaveData {
	pub name: String,
	pub money: f32,
	pub stress: f32,
	pub flirt: f32,
	pub royal: f32,
	pub drugs_taken: f32,
	pub day: f32,
	pub time: DayState,
	pub lvl_init: bool,
}

#[derive(Component)]
pub struct Cursor;

pub fn spawn_cursor(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(
		(
			SpriteBundle {
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				texture: asset_server.load("sprites/hand.png"),
				..default()
			},
			Cursor,
		)
	);
}

pub fn track_cursor(
	window_query: Query<&Window, With<PrimaryWindow>>,
	mut cursor_query: Query<&mut Transform, With<Cursor>>,
) {
	let window = window_query.get_single().unwrap();
	let mut cursor = cursor_query.get_single_mut().unwrap();

	if let Some(current_pos) = window.cursor_position() {
		cursor.translation.x = current_pos.x + 8.0;
		cursor.translation.y = current_pos.y - 8.0;
	}
}