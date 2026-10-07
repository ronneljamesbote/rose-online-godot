// ZSC object and character material. Port of object_material.wgsl.
// The including shader sets render_mode and these defines:
// ALPHA_MASK, ALPHA_BLEND, HAS_LIGHTMAP, SPECULAR, HAS_ALPHA_VALUE.

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2D base_texture : filter_linear_mipmap, repeat_enable;
uniform float alpha_cutoff = 0.5;
uniform float alpha_value = 1.0;
#ifdef HAS_LIGHTMAP
uniform sampler2D lightmap_texture : filter_linear, repeat_enable;
instance uniform vec2 lightmap_uv_offset = vec2(0.0);
instance uniform float lightmap_uv_scale = 1.0;
#endif
#ifdef SPECULAR
uniform sampler2D specular_texture : filter_linear;
#endif

void fragment() {
	vec4 color = texture(base_texture, UV);
#ifdef ALPHA_MASK
	if (color.a < alpha_cutoff) {
		discard;
	}
	color.a = 1.0;
#endif
	vec3 world_position = (INV_VIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
	vec3 world_normal = normalize((INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);

#ifdef HAS_LIGHTMAP
	vec4 lightmap = texture(lightmap_texture, (UV2 + lightmap_uv_offset) * lightmap_uv_scale);
	color.rgb *= lightmap.rgb * 2.0;
#endif
#ifdef SPECULAR
	vec3 V = normalize(CAMERA_POSITION_WORLD - world_position);
	vec3 R = reflect(-V, world_normal);
	color.rgb += color.a * texture(specular_texture, R.xy * 0.5 + vec2(0.5)).rgb;
#endif

#ifdef HAS_LIGHTMAP
	color.rgb *= rose_map_ambient;
#else
	color.rgb *= rose_character_light(world_normal);
#endif

	float fog = rose_fog_amount(VERTEX.z);
	float fog_alpha = rose_fog_alpha(fog);
	if (fog_alpha <= 0.0) {
		discard;
	}
	ALBEDO = rose_to_linear(rose_color_fog(color.rgb, fog));
#ifdef ALPHA_BLEND
#ifdef HAS_ALPHA_VALUE
	ALPHA = alpha_value * fog_alpha;
#else
	ALPHA = color.a * fog_alpha;
#endif
#endif
}
