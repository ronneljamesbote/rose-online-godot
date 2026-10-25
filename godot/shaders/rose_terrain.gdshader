// Terrain blocks of zones without a Godot scene: two tile layers blended by layer 2's
// alpha, tinted by the block's ground colour, lit by the sun. Port of terrain_material.wgsl.
// CUSTOM0 holds (layer1, layer2, rotation) / 255.
shader_type spatial;
render_mode fog_disabled, specular_disabled, ambient_light_disabled, cull_back;

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2DArray tile_textures : filter_linear_mipmap_anisotropic, repeat_enable;
// The colour the ROSE lightmap gave the block, without its shadows (zone.rs ground_tint).
uniform sampler2D tint_texture : source_color, hint_default_white, filter_linear, repeat_disable;

varying flat vec3 tile_info;

void vertex() {
	tile_info = round(CUSTOM0.xyz * 255.0);
}

void fragment() {
	int rotation = int(tile_info.z);
	vec2 layer2_uv = UV2;
	if (rotation == 2) {
		layer2_uv.x = 1.0 - layer2_uv.x;
	} else if (rotation == 3) {
		layer2_uv.y = 1.0 - layer2_uv.y;
	} else if (rotation == 4) {
		layer2_uv = vec2(1.0) - layer2_uv;
	} else if (rotation == 5) {
		layer2_uv = vec2(layer2_uv.y, 1.0 - layer2_uv.x);
	} else if (rotation == 6) {
		layer2_uv = vec2(layer2_uv.y, layer2_uv.x);
	}

	vec4 layer1 = texture(tile_textures, vec3(UV2, tile_info.x));
	vec4 layer2 = texture(tile_textures, vec3(layer2_uv, tile_info.y));
	vec3 color = mix(layer1, layer2, layer2.a).rgb;

	float fog = rose_fog_amount(VERTEX.z);
	if (rose_fog_alpha(fog) <= 0.0) {
		discard;
	}
	float fog_mix = rose_fog_mix(fog);
	vec3 base = rose_to_linear(color) * texture(tint_texture, UV).rgb * (1.0 - fog_mix);
	ALBEDO = base;
	EMISSION = base * rose_map_shade() + rose_to_linear(rose_fog_color) * fog_mix;
}

void light() {
	if (LIGHT_IS_DIRECTIONAL) {
		DIFFUSE_LIGHT += rose_map_sun() * max(dot(NORMAL, LIGHT), 0.0) * ATTENUATION;
	}
}
