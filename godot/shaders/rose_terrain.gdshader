// Terrain blocks: two tile layers blended by layer 2's alpha, times the block lightmap.
// Port of terrain_material.wgsl. CUSTOM0 holds (layer1, layer2, rotation) / 255.
shader_type spatial;
render_mode unshaded, fog_disabled, cull_back;

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2DArray tile_textures : filter_linear_mipmap, repeat_enable;
uniform sampler2D lightmap_texture : filter_linear, repeat_disable;

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
	vec4 lightmap = texture(lightmap_texture, UV);
	vec3 color = mix(layer1, layer2, layer2.a).rgb * lightmap.rgb * 2.0;
	color *= rose_map_ambient;

	float fog = rose_fog_amount(VERTEX.z);
	if (rose_fog_alpha(fog) <= 0.0) {
		discard;
	}
	ALBEDO = rose_to_linear(rose_color_fog(color, fog));
}
