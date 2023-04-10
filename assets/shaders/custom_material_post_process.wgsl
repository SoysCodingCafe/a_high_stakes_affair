#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> settings: ShaderSettings;

@group(1) @binding(3)
var<uniform> clock: Time;

struct Strength {
	wave_distortion: f32,
	hue_shift: f32,
	ripple_distortion: f32,
	contrast_distort: f32
};

struct Ripple {
	start_time: f32,
	start_x: f32,
	start_y: f32,
	velocity: f32,
};

struct ShaderSettings {
	strength: Strength,
	ripple: Ripple
};

struct Time {
	value: f32,
	padding_a: f32,
	padding_b: f32,
	padding_c: f32,
};

@fragment
fn fragment(
	@builtin(position) position: vec4<f32>,
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	// Get screen position with coordinates from 0 to 1
	let uv_0 = coords_to_viewport_uv(position.xy, view.viewport);

	var uv = uv_0;

	let offset_strength = settings.strength.wave_distortion;
	uv += 0.07 * offset_strength * wave_defect(0.0, 2.70 * uv.xy);
	// Compute an offset vector using wave_defect()
	uv -= ripple_defect(uv.xy);
	uv += 0.05 * offset_strength * wave_defect(37., 16.3 * (uv.xy - uv_0)); // Higher frequency
	uv += 0.07 * offset_strength * wave_defect(11., 3.18 * (uv.xy - 0.9 * uv_0));
	uv += 0.25 * offset_strength * wave_defect(80., 0.37 * (uv.xy - uv_0)); // Low frequency, large waves

	// Try to mitigate drifting a little
	uv = 0.5 * (uv + uv_0);
	let offset = uv - uv_0;
	let offset_length = length(offset);

	let luma_map = vec3<f32>(0.299, 0.587, 0.114);

	var rgb = textureSample(texture, our_sampler, uv).rgb;

	// Inverse tone-map
	let luminance_a = dot(luma_map, rgb);
	rgb = rgb * (1.0 + luminance_a);

	// Bass boost that bitch
	rgb = boost_contrast(settings.strength.contrast_distort, rgb);

	// Convert to HSV
	var hsv = rgb_to_hsv(rgb);
	// Perform effects, more distorted parts get more hue-shift
	hsv = vec3<f32>(
		hsv.x + offset_length * 10.0 * sin(clock.value / 360.0) * settings.strength.hue_shift,
		hsv.yz,
	);
	// Wrap colours around
	hsv = vec3<f32>(fract(hsv.x), hsv.y, hsv.z);
	// Convert back to rgb
	rgb = hsv_to_rgb(hsv);

	// Tone-map
	let luminance_b = dot(luma_map, rgb * rgb);
	rgb /= (1.0 + luminance_b);
	return vec4<f32>(rgb, 1.0);
}

fn boost_contrast(amount: f32, value: vec3<f32>) -> vec3<f32> {
	let fixed_value = 0.6;
	let rel_value = value - fixed_value;
	return fixed_value + (1.0 + amount) * rel_value;
}

fn ripple_defect(uv: vec2<f32>) -> vec2<f32> {
	let rel_uv = uv - vec2<f32>(settings.ripple.start_x, settings.ripple.start_y);
	let ripple_radius = settings.ripple.velocity * (clock.value - settings.ripple.start_time);
	let dist = length(rel_uv) - ripple_radius;
	let width = settings.strength.ripple_distortion;
	let gaub = 0.1 * exp(-(dist * dist) / (width * width));// / width; // Commented out because they'd cancel anyway
	return normalize(rel_uv) * gaub;// * settings.strength.ripple_distortion;
}

fn wave_defect(offset: f32, uv: vec2<f32>) -> vec2<f32> {
	let t = clock.value + offset;
	let dir_a = normalize(vec2<f32>(0.02 + (1.7 + uv.x) * sin(0.02 * t), -0.15 + (0.7 + 0.2 * uv.y) * sin(0.017 * t)));
	let dir_b = normalize(vec2<f32>(0.01 + (1.7 - uv.y) * cos(0.02 * t),  0.17 + (0.7 + 0.2 * uv.x) * sin(0.017 * t)));
	return vec2<f32>(
		sin(5.1 * dot(uv.xy, dir_a) + 4.7 * dot(uv.yx, dir_b) + 0.28 * t),
		cos(5.2 * dot(uv.yx, dir_a) + 4.3 * dot(uv.xy, dir_b) + 0.25 * t)
	);
}

fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
	var P   = vec4<f32>(rgb.gb, 0.0, -1.0/3.0);
	if (rgb.g < rgb.b) {
		P   = vec4<f32>(rgb.bg, -1.0, 2.0/3.0);
	}
	var Q   = vec4<f32>(rgb.r, P.yzx);
	if (rgb.r < P.x) {
		Q   = vec4<f32>(P.xyw, rgb.r);
	}
	let C   = Q.x - min(Q.w, Q.y);
	let H   = abs((Q.w - Q.y) / (6.0 * C + 0.001) + Q.z);
	let HCV = vec3<f32>(H, C, Q.x);
	let S   = HCV.y / (HCV.z + 0.001);
	return vec3<f32>(HCV.x, S, HCV.z);
}

fn hsv_to_rgb(HSV: vec3<f32>) -> vec3<f32> {
	let H   = HSV.x;
	let R   = abs(H * 6.0 - 3.0) - 1.0;
	let G   = 2.0 - abs(H * 6.0 - 2.0);
	let B   = 2.0 - abs(H * 6.0 - 4.0);
	let RGB = clamp( vec3<f32>(R,G,B), vec3<f32>(0.0), vec3<f32>(1.0) );
	return ((RGB - vec3<f32>(1.0)) * HSV.y + vec3<f32>(1.0)) * HSV.z;
}
