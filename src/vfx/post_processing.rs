//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use bevy::{
	prelude::*,
	reflect::TypeUuid,
	render::{
		render_resource::{
			AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
			TextureUsages, ShaderType,
		},
		texture::BevyDefault,
		view::RenderLayers,
	},
	sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

use crate::game::states::GameState;

use super::{PostProcessingSettings, PostProcessingMaterialHandle};

#[derive(Debug, Default)]
pub struct VFXPlugin {}

impl Plugin for VFXPlugin {
	fn build(&self, app: &mut App) {
		app
		.init_resource::<PostProcessingSettings>()
		.add_startup_system(vfx_setup.in_set(super::VFXChangeSystemSet))
		.add_system(update_settings.after(super::VFXChangeSystemSet))
		.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default());
	}
}

pub(crate) fn vfx_setup(
	mut commands: Commands,
	windows: Query<&Window>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
	mut images: ResMut<Assets<Image>>,
) {
	// This assumes we only have a single window
	let window = windows.single();

	let size = Extent3d {
		width: window.resolution.physical_width(),
		height: window.resolution.physical_height(),
		..default()
	};

	// This is the texture that will be rendered to.
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::bevy_default(),
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		},
		..default()
	};

	// fill image.data with zeroes
	image.resize(size);

	let image_handle = images.add(image);

	commands.insert_resource(super::RenderTargetImage {image: image_handle.clone()});

	// This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
	let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

	let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
		size.width as f32,
		size.height as f32,
	))));

	// This material has the texture that has been rendered.
	let material_handle = post_processing_materials.add(PostProcessingMaterial::new(image_handle.clone()));

	commands.insert_resource(PostProcessingMaterialHandle { handle: material_handle.clone() });

	// Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: quad_handle.into(),
			material: material_handle,
			transform: Transform {
				translation: Vec3::new(0.0, 0.0, 1.5),
				..default()
			},
			..default()
		},
		post_processing_pass_layer,
	));

	// The post-processing pass camera.
	commands.spawn((
		Camera2dBundle {
			camera: Camera {
				// renders after the first main camera which has default value: 0.
				order: 1,
				..default()
			},
			..Camera2dBundle::default()
		},
		UiCameraConfig { show_ui: false },
		post_processing_pass_layer,
	));
}

fn update_settings(
	mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
	handle: Res<PostProcessingMaterialHandle>,
	settings: Res<PostProcessingSettings>,
	time: Res<Time>
) {
	if let Some(material) = post_processing_materials.get_mut(&handle) {
		material.settings = settings.clone();
		material.time.time = time.elapsed_seconds();
	}
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
	/// In this example, this image will be the result of the main camera.
	#[texture(0)]
	#[sampler(1)]
	source_image: Handle<Image>,
	#[uniform(2)]
	settings: PostProcessingSettings,
	#[uniform(3)]
	time: PostProcessingMaterialTime,
}

#[derive(ShaderType, Default, Clone, Copy)]
struct PostProcessingMaterialTime {
	time: f32,
	_wasm_padding_a: f32,
	_wasm_padding_b: f32,
	_wasm_padding_c: f32, // Padded to 16 bytes to make WASM happy
}

impl PostProcessingMaterial {
	pub fn new(source_image: Handle<Image>) -> PostProcessingMaterial {
		PostProcessingMaterial { 
			source_image, 
			settings: PostProcessingSettings::default(), 
			time: PostProcessingMaterialTime::default()
		}
	}
}

impl Material2d for PostProcessingMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material_post_process.wgsl".into()
	}
}
