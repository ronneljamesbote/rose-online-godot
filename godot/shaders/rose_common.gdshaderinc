// Zone lighting and fog shared by every ROSE material.
// Port of rose-offline-client's render/shaders/zone_lighting.wgsl.
//
// ROSE textures and lighting were made for gamma-space rendering. Textures are sampled
// raw (no source_color hint), all maths happens on those raw values like the original
// client, and the result is converted to linear at the very end so Godot's linear->sRGB
// output gives back the original colour.

global uniform vec3 rose_map_ambient;
global uniform vec3 rose_character_ambient;
global uniform vec3 rose_character_diffuse;
global uniform vec3 rose_light_direction;
global uniform vec3 rose_fog_color;
global uniform float rose_fog_density;

const float ROSE_FOG_MIN_DENSITY = 0.0;
const float ROSE_FOG_MAX_DENSITY = 0.75;
const float ROSE_FOG_ALPHA_START = 0.85;
const float ROSE_FOG_ALPHA_END = 0.98;

vec3 rose_to_linear(vec3 c) {
	c = clamp(c, vec3(0.0), vec3(1.0));
	return mix(c / 12.92, pow((c + 0.055) / 1.055, vec3(2.4)), step(vec3(0.04045), c));
}

float rose_fog_amount(float view_z) {
	return clamp(1.0 - exp2(-rose_fog_density * rose_fog_density * view_z * view_z * 1.442695), 0.0, 1.0);
}

vec3 rose_color_fog(vec3 color, float fog_amount) {
	return mix(color, rose_fog_color, clamp(fog_amount, ROSE_FOG_MIN_DENSITY, ROSE_FOG_MAX_DENSITY));
}

// 1.0 = fully visible, 0.0 = fogged out (the caller discards).
float rose_fog_alpha(float fog_amount) {
	if (fog_amount >= ROSE_FOG_ALPHA_END) {
		return 0.0;
	}
	if (fog_amount >= ROSE_FOG_ALPHA_START) {
		return 1.0 - (fog_amount - ROSE_FOG_ALPHA_START) / (ROSE_FOG_ALPHA_END - ROSE_FOG_ALPHA_START);
	}
	return 1.0;
}

vec3 rose_character_light(vec3 world_normal) {
	return clamp(rose_character_ambient + rose_character_diffuse * clamp(dot(world_normal, rose_light_direction), 0.0, 1.0), vec3(0.0), vec3(1.0));
}
