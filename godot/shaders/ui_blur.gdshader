// Blurs the world behind a window (the "glass" look), inside a rounded rectangle.
shader_type canvas_item;

uniform sampler2D screen_tex : hint_screen_texture, filter_linear_mipmap;
uniform float lod = 3.0;
uniform float radius = 10.0;
uniform vec2 size = vec2(100.0, 100.0);

void fragment() {
	vec2 p = (UV - 0.5) * size;
	vec2 q = abs(p) - size * 0.5 + radius;
	float d = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - radius;
	float mask = clamp(0.5 - d, 0.0, 1.0);
	vec2 px = SCREEN_PIXEL_SIZE * exp2(lod) * 0.6;
	vec3 c = textureLod(screen_tex, SCREEN_UV, lod).rgb * 0.4;
	c += textureLod(screen_tex, SCREEN_UV + vec2(px.x, px.y), lod).rgb * 0.15;
	c += textureLod(screen_tex, SCREEN_UV + vec2(-px.x, px.y), lod).rgb * 0.15;
	c += textureLod(screen_tex, SCREEN_UV + vec2(px.x, -px.y), lod).rgb * 0.15;
	c += textureLod(screen_tex, SCREEN_UV + vec2(-px.x, -px.y), lod).rgb * 0.15;
	COLOR = vec4(c, mask);
}
