// Effect particles and meshes (EFT files), ported from rose-offline-client's particle.wgsl
// and effect_mesh_material.wgsl. effect.rs writes a small shader per render state that sets
// render_mode and these defines, then includes this file:
//   PARTICLE         camera-facing quads from a MultiMesh (instance custom data: size x,
//                    size y, rotation, texture atlas index)
//   MESH_ANIMATION   vertex animation from a ZMO baked into animation_texture
//   COLOR_OPAQUE, COLOR_MASK, COLOR_ONE, COLOR_SQUARED, COLOR_SCREEN   how the colour
//                    and alpha are written so Godot's blend modes match the ROSE blend
//                    factors (default: colour and alpha as they are)
#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2D base_texture : filter_linear_mipmap, repeat_enable;

#ifdef PARTICLE
// 0: faces the camera, 1: lies flat, 2: turns about the Y axis only.
uniform int billboard = 0;
uniform vec2 atlas_size = vec2(1.0);
varying vec4 tint;
#endif

#ifdef MESH_ANIMATION
uniform sampler2D animation_texture : filter_nearest;
// Bits 1 position, 2 normal, 4 uv, 8 alpha.
uniform int animation_flags = 0;
uniform int animation_frames = 1;
uniform int current_frame = 0;
uniform int next_frame = 0;
uniform float next_weight = 0.0;
uniform float animation_alpha = 1.0;
#endif

void vertex() {
#ifdef PARTICLE
	vec3 centre = MODEL_MATRIX[3].xyz;
	vec3 right = vec3(1.0, 0.0, 0.0);
	vec3 up = vec3(0.0, 0.0, 1.0);
	if (billboard == 0) {
		right = normalize(INV_VIEW_MATRIX[0].xyz);
		up = normalize(INV_VIEW_MATRIX[1].xyz);
	} else if (billboard == 2) {
		right = normalize(vec3(INV_VIEW_MATRIX[0].x, 0.0, INV_VIEW_MATRIX[0].z));
		up = vec3(0.0, 1.0, 0.0);
	}
	float c = cos(INSTANCE_CUSTOM.z);
	float s = sin(INSTANCE_CUSTOM.z);
	vec2 corner = vec2(c * VERTEX.x + s * VERTEX.y, -s * VERTEX.x + c * VERTEX.y);
	vec3 world = centre + right * corner.x * INSTANCE_CUSTOM.x + up * corner.y * INSTANCE_CUSTOM.y;
	POSITION = PROJECTION_MATRIX * VIEW_MATRIX * vec4(world, 1.0);
	int columns = max(int(atlas_size.x), 1);
	int cells = max(columns * max(int(atlas_size.y), 1), 1);
	int index = int(abs(INSTANCE_CUSTOM.w)) % cells;
	UV = (vec2(float(index % columns), float(index / columns)) + UV) / atlas_size;
	tint = COLOR;
#endif

#ifdef MESH_ANIMATION
	if ((animation_flags & 1) != 0) {
		vec4 a = texelFetch(animation_texture, ivec2(current_frame, VERTEX_ID), 0);
		vec4 b = texelFetch(animation_texture, ivec2(next_frame, VERTEX_ID), 0);
		VERTEX = mix(a.xyz, b.xyz, next_weight);
	}
	if ((animation_flags & 4) != 0) {
		vec4 a0 = texelFetch(animation_texture, ivec2(current_frame, VERTEX_ID), 0);
		vec4 b0 = texelFetch(animation_texture, ivec2(next_frame, VERTEX_ID), 0);
		vec4 a1 = texelFetch(animation_texture, ivec2(current_frame + animation_frames, VERTEX_ID), 0);
		vec4 b1 = texelFetch(animation_texture, ivec2(next_frame + animation_frames, VERTEX_ID), 0);
		UV = vec2(mix(a0.w, b0.w, next_weight), mix(a1.w, b1.w, next_weight));
	}
#endif
}

void fragment() {
	vec4 color = texture(base_texture, UV);
#ifdef PARTICLE
	color *= tint;
#endif
#ifdef MESH_ANIMATION
	if ((animation_flags & 8) != 0) {
		color.a *= animation_alpha;
	}
#endif

#if defined(COLOR_OPAQUE)
	ALBEDO = rose_to_linear(color.rgb);
#elif defined(COLOR_MASK)
	ALBEDO = rose_to_linear(color.rgb);
	ALPHA = color.a;
	ALPHA_SCISSOR_THRESHOLD = 0.5;
#elif defined(COLOR_ONE)
	// One, One: added whatever its alpha.
	ALBEDO = rose_to_linear(color.rgb);
	ALPHA = 1.0;
#elif defined(COLOR_SQUARED)
	// SrcColor, One: the colour weighted by itself.
	ALBEDO = rose_to_linear(color.rgb * color.rgb);
	ALPHA = 1.0;
#elif defined(COLOR_SCREEN)
	// Destination weighted by one minus the source colour (premultiplied blend).
	ALBEDO = rose_to_linear(color.rgb * color.a);
	ALPHA = max(color.r, max(color.g, color.b));
#else
	ALBEDO = rose_to_linear(color.rgb);
	ALPHA = color.a;
#endif
}
