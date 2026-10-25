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
// The sun (a DirectionalLight3D, RoseSun): its colour and strength, and how much of the
// ground light it gives (0 = none, no shadows; 0.4 = shade is 60 % as bright). See rose_map_shade().
global uniform vec3 rose_sun_color;
global uniform float rose_sun_strength;
global uniform float rose_shadow_darkness;
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

// Real-time light (linear). The lit shaders put the light that reaches everywhere in
// EMISSION and add the sun's share in light(), where Godot gives the shadow (ATTENUATION).
// Lit fully by the sun and facing it, they look as the unlit ROSE ground light did.

// Ground and buildings out of the sun: the ground light minus the sun's share.
vec3 rose_map_shade() {
	return rose_to_linear(rose_map_ambient * (1.0 - rose_shadow_darkness));
}

// What the sun adds to ground and buildings facing it straight on.
vec3 rose_map_sun() {
	return max(rose_to_linear(rose_map_ambient) - rose_map_shade(), vec3(0.0)) * rose_to_linear(rose_sun_color) * rose_sun_strength;
}

// What the sun adds to characters facing it straight on: ROSE lit them by
// ambient + diffuse * facing, capped at 1.
vec3 rose_character_sun() {
	return max(rose_to_linear(min(rose_character_ambient + rose_character_diffuse, vec3(1.0))) - rose_to_linear(rose_character_ambient), vec3(0.0));
}

// The fog's share of a pixel, and the colour it adds (linear).
float rose_fog_mix(float fog_amount) {
	return clamp(fog_amount, ROSE_FOG_MIN_DENSITY, ROSE_FOG_MAX_DENSITY);
}
