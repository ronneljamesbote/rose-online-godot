// Refine glow on equipment (CObjAVT::CreateGradeEffect in the original client). The item is
// drawn again into the glow buffer as texture x grade colour (ZZ_GLOW_TEXTURE), and that
// buffer is blurred and added to the screen: the item brightens in the colour of its grade
// and a soft halo of it spills around it.
// GLOW_SHELL: the halo, the mesh pushed out a little along its normals and faded towards
// the inside. Without it: the brightening on the item itself.

#include "res://shaders/rose_common.gdshaderinc"

uniform sampler2D base_texture : filter_linear_mipmap, repeat_enable;
uniform vec3 glow_colour = vec3(1.0);
uniform bool use_texture_alpha = false;
uniform float alpha_cutoff = 0.5;
#ifdef GLOW_SHELL
uniform float shell_width = 0.025;
#endif

void vertex() {
#ifdef GLOW_SHELL
	VERTEX += NORMAL * shell_width;
#endif
}

void fragment() {
	vec4 color = texture(base_texture, UV);
	if (use_texture_alpha && color.a < alpha_cutoff) {
		discard;
	}
	float strength = 0.65;
#ifdef GLOW_SHELL
	// Facing the camera: the inside of the halo, hidden by the item; at the outline: the
	// blur's edge. Brightest in between.
	float facing = abs(dot(normalize(NORMAL), normalize(VIEW)));
	strength = 0.6 * smoothstep(0.0, 0.5, facing) * (1.0 - facing * 0.6);
	// The blur averages the texture, so the halo takes its brightness, not its pattern.
	color.rgb = vec3(0.35) + color.rgb * 0.65;
#endif
	float fog = rose_fog_amount(VERTEX.z);
	ALBEDO = rose_to_linear(color.rgb * glow_colour) * strength * (1.0 - fog);
}
