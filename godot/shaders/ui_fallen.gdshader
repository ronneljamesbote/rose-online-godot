// The world behind the death screen: drained of colour, darker at the edges.
shader_type canvas_item;

uniform sampler2D screen_tex : hint_screen_texture, filter_linear;
uniform float amount = 1.0;

void fragment() {
	vec3 c = texture(screen_tex, SCREEN_UV).rgb;
	float grey = dot(c, vec3(0.299, 0.587, 0.114));
	vec3 tinted = mix(c, vec3(grey) * vec3(1.0, 0.92, 0.92), 0.85 * amount);
	float d = distance(SCREEN_UV, vec2(0.5));
	float vignette = smoothstep(0.25, 0.85, d);
	COLOR = vec4(tinted * (1.0 - 0.65 * vignette * amount) * (1.0 - 0.15 * amount), 1.0);
}
