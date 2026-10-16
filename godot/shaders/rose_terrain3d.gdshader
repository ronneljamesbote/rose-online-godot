shader_type spatial;
// Terrain3D shader with the ROSE ground look (see zone_scene.rs). The vertex part is
// Terrain3D 1.0.2's own and must stay as it is; fragment() is a port of
// rose_terrain.gdshader: each 10 m tile (4 x 4 vertices) draws texture layer 1 with layer 2
// on top by layer 2's alpha (times the control blend, so Terrain3D's paint tools work),
// layer 2 flipped or turned by the control's angle bits (ROSE rotation codes 0-6), all
// times the block's lightmap.
render_mode unshaded, fog_disabled, cull_back, depth_draw_opaque, skip_vertex_transform;

/* The terrain depends on this shader to function. Don't change most things in vertex() or 
 * terrain normal calculations in fragment(). You probably only want to customize the 
 * material calculation and PBR application in fragment().
 *
 * Uniforms that begin with _ are private and will not display in the inspector. However, 
 * you can set them via code. You are welcome to create more of your own hidden uniforms.
 *
 * This system only supports albedo, height, normal, roughness. Most textures don't need the other
 * PBR channels. Height can be used as an approximation for AO. For the rare textures do need
 * additional channels, you can add maps for that one texture. e.g. an emissive map for lava.
 *
 */

// Defined Constants
#define SKIP_PASS 0
#define VERTEX_PASS 1
#define FRAGMENT_PASS 2
#define COLOR_MAP_DEF vec4(1.0, 1.0, 1.0, 0.5)
#define DIV_255 0.003921568627450 // 1. / 255.
#define DIV_1024 0.0009765625 // 1. / 1024.
#define TAU_16TH -0.392699081698724 // -TAU / 16.

// Inline Functions
#define DECODE_BLEND(control) float(control >> 14u & 0xFFu) * DIV_255
#define DECODE_AUTO(control) bool(control & 0x1u)
#define DECODE_BASE(control) int(control >> 27u & 0x1Fu)
#define DECODE_OVER(control) int(control >> 22u & 0x1Fu)
#define DECODE_ANGLE(control) float(control >>10u & 0xFu) * TAU_16TH
// This math recreates the scale value directly rather than using an 8 float const array.
#define DECODE_SCALE(control) (0.9 - float(((control >>7u & 0x7u) + 3u) % 8u + 1u) * 0.1)
#define DECODE_HOLE(control) bool(control >>2u & 0x1u)

#if CURRENT_RENDERER == RENDERER_COMPATIBILITY
    #define fma(a, b, c) ((a) * (b) + (c))
    #define dFdxCoarse(a) dFdx(a)
    #define dFdyCoarse(a) dFdy(a)
#endif

// Private uniforms
uniform vec3 _camera_pos = vec3(0.f);
uniform float _mesh_size = 48.f;
uniform uint _background_mode = 1u; // NONE = 0, FLAT = 1, NOISE = 2
uniform uint _mouse_layer = 0x80000000u; // Layer 32
uniform float _vertex_spacing = 1.0;
uniform float _vertex_density = 1.0; // = 1./_vertex_spacing
uniform float _region_size = 1024.0;
uniform float _region_texel_size = 0.0009765625; // = 1./region_size
uniform int _region_map_size = 32;
uniform int _region_map[1024];
uniform vec2 _region_locations[1024];
uniform float _texture_normal_depth_array[32];
uniform float _texture_ao_strength_array[32];
uniform float _texture_roughness_mod_array[32];
uniform float _texture_uv_scale_array[32];
uniform vec2 _texture_detile_array[32];
uniform vec4 _texture_color_array[32];
uniform highp sampler2DArray _height_maps : repeat_disable;
uniform highp sampler2DArray _control_maps : repeat_disable;
#define FILTER_LINEAR
uniform highp sampler2DArray _color_maps : source_color, filter_linear_mipmap_anisotropic, repeat_disable;
// Sampled raw like every ROSE texture (see rose_common.gdshaderinc), so no source_color.
uniform highp sampler2DArray _texture_array_albedo : filter_linear_mipmap, repeat_enable;
uniform highp sampler2DArray _texture_array_normal : hint_normal, filter_linear_mipmap_anisotropic, repeat_enable;


// Public uniforms
uniform float blend_sharpness : hint_range(0, 1) = 0.5;
uniform bool flat_terrain_normals = false;
uniform bool enable_projection = true;
uniform float projection_threshold : hint_range(0.0, 0.99, 0.01) = 0.8;

uniform float mipmap_bias : hint_range(0.5, 1.5, 0.01) = 1.0;
uniform float depth_blur : hint_range(0.0, 35.0, 0.1) = 0.0;
uniform float bias_distance : hint_range(0.0, 16384.0, 0.1) = 512.0;

uniform bool enable_macro_variation = true;
uniform vec3 macro_variation1 : source_color = vec3(1.);
uniform vec3 macro_variation2 : source_color = vec3(1.);
uniform float macro_variation_slope : hint_range(0., 1.)  = 0.333;

// Generic noise at 2 scales, which can be used for anything 
uniform highp sampler2D noise_texture : source_color, filter_linear_mipmap_anisotropic, repeat_enable;

uniform float noise1_scale : hint_range(0.001, 1.) = 0.04; // Used for macro variation 1. Scaled up 10x
uniform float noise1_angle : hint_range(0, 6.283) = 0.;
uniform vec2 noise1_offset = vec2(0.5);
uniform float noise2_scale : hint_range(0.001, 1.) = 0.076;	// Used for macro variation 2. Scaled up 10x

// Varyings & Types

struct material {
	vec4 albedo_height;
	vec4 normal_rough;
	float normal_map_depth;
	float ao_strength;
	float total_weight;
};

varying float v_vertex_xz_dist;
varying vec3 v_vertex;

////////////////////////
// Vertex
////////////////////////

// Takes in world space XZ (UV) coordinates & search depth (only applicable for background mode none)
// Returns ivec3 with:
// XY: (0 to _region_size - 1) coordinates within a region
// Z: layer index used for texturearrays, -1 if not in a region
ivec3 get_index_coord(const vec2 uv, const int search) {
	vec2 r_uv = round(uv);
	vec2 o_uv = mod(r_uv,_region_size);
	ivec2 pos;
	int bounds, layer_index = -1;
	for (int i = -1; i < clamp(search, SKIP_PASS, FRAGMENT_PASS); i++) {
		if ((layer_index == -1 && _background_mode == 0u ) || i < 0) {
			r_uv -= i == -1 ? vec2(0.0) : vec2(float(o_uv.x <= o_uv.y), float(o_uv.y <= o_uv.x));
			pos = ivec2(floor((r_uv) * _region_texel_size)) + (_region_map_size / 2);
			bounds = int(uint(pos.x | pos.y) < uint(_region_map_size));
			layer_index = (_region_map[ pos.y * _region_map_size + pos.x ] * bounds - 1);
		}
	}
	return ivec3(ivec2(mod(r_uv,_region_size)), layer_index);
}

// Takes in descaled (world_space / region_size) world to region space XZ (UV2) coordinates, returns vec3 with:
// XY: (0. to 1.) coordinates within a region
// Z: layer index used for texturearrays, -1 if not in a region
vec3 get_index_uv(const vec2 uv2) {
	ivec2 pos = ivec2(floor(uv2)) + (_region_map_size / 2);
	int bounds = int(uint(pos.x | pos.y) < uint(_region_map_size));
	int layer_index = _region_map[ pos.y * _region_map_size + pos.x ] * bounds - 1;
	return vec3(uv2 - _region_locations[layer_index], float(layer_index));
}

void vertex() {
	// Get vertex of flat plane in world coordinates and set world UV
	v_vertex = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;

	// Camera distance to vertex on flat plane
	v_vertex_xz_dist = length(v_vertex.xz - _camera_pos.xz);

	// Geomorph vertex, set end and start for linear height interpolate
	float scale = MODEL_MATRIX[0][0];
	float vertex_lerp = smoothstep(0.55, 0.95, (v_vertex_xz_dist / scale - _mesh_size - 4.0) / (_mesh_size - 2.0));
	vec2 v_fract = fract(VERTEX.xz * 0.5) * 2.0;
	// For LOD0 morph from a regular grid to an alternating grid to align with LOD1+
	vec2 shift = (scale < _vertex_spacing + 1e-6) ? // LOD0 or not
		// Shift from regular to symetric
		mix(v_fract, vec2(v_fract.x, -v_fract.y),
			round(fract(round(mod(v_vertex.z * _vertex_density, 4.0)) *
			round(mod(v_vertex.x * _vertex_density, 4.0)) * 0.25))
			) :
		// Symetric shift
		v_fract * round((fract(v_vertex.xz * 0.25 / scale) - 0.5) * 4.0);
	vec2 start_pos = v_vertex.xz * _vertex_density;
	vec2 end_pos = (v_vertex.xz - shift * scale) * _vertex_density;
	v_vertex.xz -= shift * scale * vertex_lerp;

	// UV coordinates in world space. Values are 0 to _region_size within regions
	UV = v_vertex.xz * _vertex_density;

	// UV coordinates in region space + texel offset. Values are 0 to 1 within regions
	UV2 = fma(UV, vec2(_region_texel_size), vec2(0.5 * _region_texel_size));

	// Discard vertices for Holes. 1 lookup
	ivec3 v_region = get_index_coord(start_pos, VERTEX_PASS);
	uint control = floatBitsToUint(texelFetch(_control_maps, v_region, 0)).r;
	bool hole = DECODE_HOLE(control);

	// Show holes to all cameras except mouse camera (on exactly 1 layer)
	if ( !(CAMERA_VISIBLE_LAYERS == _mouse_layer) && 
			(hole || (_background_mode == 0u && v_region.z == -1))) {
		v_vertex.x = 0. / 0.;
	} else {		
		// Set final vertex height & calculate vertex normals. 3 lookups
		ivec3 coord_a = get_index_coord(start_pos, VERTEX_PASS);
		ivec3 coord_b = get_index_coord(end_pos, VERTEX_PASS);
		float h = mix(texelFetch(_height_maps, coord_a, 0).r,texelFetch(_height_maps, coord_b, 0).r,vertex_lerp);
		v_vertex.y = h;
	}

	// Convert model space to view space w/ skip_vertex_transform render mode
	VERTEX = (VIEW_MATRIX * vec4(v_vertex, 1.0)).xyz;
	NORMAL = normalize((MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz);
	BINORMAL = normalize((MODELVIEW_MATRIX * vec4(BINORMAL, 0.0)).xyz);
	TANGENT = normalize((MODELVIEW_MATRIX * vec4(TANGENT, 0.0)).xyz);
}

#include "res://shaders/rose_common.gdshaderinc"

// The ROSE blocks' lightmaps (one 160 m block per layer) and, per block of the 64 x 64
// grid, its layer + 1 (0: none). Set by RoseTerrainLook.
uniform highp sampler2DArray rose_lightmaps : filter_linear, repeat_disable;
uniform highp sampler2D rose_lightmap_index : filter_nearest, repeat_disable;

////////////////////////
// Fragment
////////////////////////

void fragment() {
	// Terrain3D adds editor code (the brush preview) that reads these.
	vec2 uv = UV;
	vec2 uv2 = UV2;
	vec2 world = v_vertex.xz;
	// ROSE block grid: block x from world x, block y from world z (see zone.rs block_origin).
	vec2 block_f = vec2(world.x / 160.0, 65.0 + world.y / 160.0);
	ivec2 block = ivec2(floor(block_f));
	vec2 local = fract(block_f) * 160.0;

	// Each 10 m tile's textures come from the control map at a vertex inside the tile.
	vec2 tile_vertex = floor(world / 10.0) * 4.0 + 1.0;
	uint control = floatBitsToUint(texelFetch(_control_maps, get_index_coord(tile_vertex, FRAGMENT_PASS), 0)).r;
	int layer1 = DECODE_BASE(control);
	int layer2 = DECODE_OVER(control);
	uint rotation = control >> 10u & 0xFu;
	float blend = DECODE_BLEND(control);

	vec2 tile_uv = local / 10.0;
	vec2 uv_dx = dFdx(tile_uv);
	vec2 uv_dy = dFdy(tile_uv);
	tile_uv = fract(tile_uv);
	vec2 layer2_uv = tile_uv;
	vec2 dx2 = uv_dx;
	vec2 dy2 = uv_dy;
	if (rotation == 2u) {
		layer2_uv.x = 1.0 - layer2_uv.x; dx2.x = -dx2.x; dy2.x = -dy2.x;
	} else if (rotation == 3u) {
		layer2_uv.y = 1.0 - layer2_uv.y; dx2.y = -dx2.y; dy2.y = -dy2.y;
	} else if (rotation == 4u) {
		layer2_uv = vec2(1.0) - layer2_uv; dx2 = -dx2; dy2 = -dy2;
	} else if (rotation == 5u) {
		layer2_uv = vec2(layer2_uv.y, 1.0 - layer2_uv.x); dx2 = vec2(dx2.y, -dx2.x); dy2 = vec2(dy2.y, -dy2.x);
	} else if (rotation == 6u) {
		layer2_uv = vec2(layer2_uv.y, layer2_uv.x); dx2 = dx2.yx; dy2 = dy2.yx;
	}

	vec4 color1 = textureGrad(_texture_array_albedo, vec3(tile_uv, float(layer1)), uv_dx, uv_dy);
	vec4 color2 = textureGrad(_texture_array_albedo, vec3(layer2_uv, float(layer2)), dx2, dy2);

	vec3 light = vec3(0.5);
	bool in_grid = block.x >= 0 && block.y >= 0 && block.x < 64 && block.y < 64;
	int lightmap = in_grid ? int(round(texelFetch(rose_lightmap_index, block, 0).r * 255.0)) : 0;
	if (lightmap > 0) {
		light = texture(rose_lightmaps, vec3(local / 160.0, float(lightmap - 1))).rgb;
	}
	vec3 rose_color = mix(color1, color2, color2.a * blend).rgb * light * 2.0;
	rose_color *= rose_map_ambient;

	float fog = rose_fog_amount(VERTEX.z);
	if (rose_fog_alpha(fog) <= 0.0) {
		discard;
	}
	ALBEDO = rose_to_linear(rose_color_fog(rose_color, fog));
}
