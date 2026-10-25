// ZSC object and character material. Port of object_material.wgsl, lit by the sun.
// The including shader (one per render state in shaders/objects/, see material.rs) sets
// render_mode and ALPHA_MASK or ALPHA_BLEND. Zone models use the ground light (map_light),
// characters, monsters, NPCs and items the character light.

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2D base_texture : filter_linear_mipmap_anisotropic, repeat_enable;
uniform float alpha_cutoff = 0.5;
uniform bool use_alpha_value = false;
uniform float alpha_value = 1.0;
uniform bool specular = false;
uniform sampler2D specular_texture : filter_linear;
uniform bool map_light = false;

void fragment() {
	vec4 color = texture(base_texture, UV);
#ifdef ALPHA_MASK
	if (color.a < alpha_cutoff) {
		discard;
	}
	color.a = 1.0;
#endif
	if (specular) {
		vec3 world_position = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
		vec3 world_normal = normalize((INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
		vec3 V = normalize(CAMERA_POSITION_WORLD - world_position);
		vec3 R = reflect(-V, world_normal);
		color.rgb += color.a * texture(specular_texture, R.xy * 0.5 + vec2(0.5)).rgb;
	}

	float fog = rose_fog_amount(VERTEX.z);
	float fog_alpha = rose_fog_alpha(fog);
	if (fog_alpha <= 0.0) {
		discard;
	}
	float fog_mix = rose_fog_mix(fog);
	vec3 base = rose_to_linear(color.rgb) * (1.0 - fog_mix);
	ALBEDO = base;
	EMISSION = base * (map_light ? rose_map_shade() : rose_to_linear(rose_character_ambient)) + rose_to_linear(rose_fog_color) * fog_mix;
#ifdef ALPHA_BLEND
	ALPHA = (use_alpha_value ? alpha_value : color.a) * fog_alpha;
#endif
}

void light() {
	if (LIGHT_IS_DIRECTIONAL) {
		vec3 sun = map_light ? rose_map_sun() : rose_character_sun();
		DIFFUSE_LIGHT += sun * max(dot(NORMAL, LIGHT), 0.0) * ATTENUATION;
	}
}
