use bevy::{prelude::{Handle, Image, Resource, Deref, DerefMut, SystemSet}, render::render_resource::ShaderType};

use self::post_processing::PostProcessingMaterial;

pub mod post_processing;
pub mod vfx_triggers;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct VFXChangeSystemSet; // System set for all the systems which edit Res<PostProcessingSettings>

#[derive(Resource, Deref, DerefMut)]
pub struct RenderTargetImage { // A handle to the image which should be rendered to in order to get post-processing
	pub image: Handle<Image>
}

#[derive(Resource, Deref, DerefMut)]
struct PostProcessingMaterialHandle {
	pub handle: Handle<PostProcessingMaterial>
}

#[derive(ShaderType, Default, Resource, Clone, Copy)]
pub struct PostProcessingSettings { // Each field is a multiple of 16 bytes
	pub strength: PostProcessingStrength,
	pub ripple: PostProcessingRipple,
}

#[derive(ShaderType, Clone, Copy)]
pub struct PostProcessingStrength {
	pub wave_distort: f32,
	pub hue_shift: f32,
	pub ripple_distort: f32,
	pub contrast_distort: f32,
}

impl Default for PostProcessingStrength {
	fn default() -> Self {
		Self {
			wave_distort: 0.0,
			hue_shift: 0.5,
			ripple_distort: 0.02,
			contrast_distort: 0.0
		}
	}
}

#[derive(ShaderType, Clone, Copy)]
pub struct PostProcessingRipple {
	pub start_time: f32,
	pub start_x: f32,
	pub start_y: f32,
	pub velocity: f32,
}

impl Default for PostProcessingRipple {
	fn default() -> Self {
		Self {
			start_time: -100.0,
			start_x: 0.5,
			start_y: 0.5,
			velocity: 0.2,
		}
	}
}
