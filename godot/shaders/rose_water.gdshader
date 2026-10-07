// Animated additive water planes. Port of water_material.wgsl (25 frames at 10 fps).
shader_type spatial;
render_mode unshaded, fog_disabled, blend_add, depth_draw_never, cull_disabled;

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2DArray water_textures : filter_linear_mipmap, repeat_enable;

void fragment() {
	float t = TIME * 10.0;
	float current = mod(floor(t), 25.0);
	float next = mod(current + 1.0, 25.0);
	vec4 color = mix(texture(water_textures, vec3(UV, current)), texture(water_textures, vec3(UV, next)), fract(t));
	color.rgb *= rose_map_ambient;

	// Additive water gets alpha fog only, no colour fog.
	float fog_alpha = rose_fog_alpha(rose_fog_amount(VERTEX.z));
	if (fog_alpha <= 0.0) {
		discard;
	}
	ALBEDO = rose_to_linear(color.rgb);
	ALPHA = color.a * fog_alpha;
}
