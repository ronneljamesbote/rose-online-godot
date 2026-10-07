// Sky dome that follows the camera and sits behind everything. Port of sky_material.wgsl.
shader_type spatial;
render_mode unshaded, fog_disabled, cull_disabled, depth_draw_never, skip_vertex_transform;

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2D texture_day : filter_linear_mipmap, repeat_enable;
uniform sampler2D texture_night : filter_linear_mipmap, repeat_enable;
global uniform float rose_sky_day_weight;

void vertex() {
	vec3 view_direction = mat3(VIEW_MATRIX) * (mat3(MODEL_MATRIX) * VERTEX);
	POSITION = PROJECTION_MATRIX * vec4(view_direction, 1.0);
	// Godot uses reversed Z: a depth just above 0 is the far plane.
	POSITION.z = POSITION.w * 0.000001;
}

void fragment() {
	vec3 day = texture(texture_day, UV).rgb;
	vec3 night = texture(texture_night, UV).rgb;
	ALBEDO = rose_to_linear(mix(night, day, rose_sky_day_weight));
}
