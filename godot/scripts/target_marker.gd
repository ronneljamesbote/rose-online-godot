## The ring on the ground under the selected monster, NPC or player. One Decal, moved to
## the current target every frame, so it lies on the terrain, stairs and floors under the
## target's feet. Its size follows the target's model, its colour what the target is:
## red a monster (or a player we may fight in PvP), green an NPC, blue another player.
## It turns slowly and pulses gently.
##
## Characters, monsters and NPCs draw on render layer 2 (net_entity.gd), which the decal
## skips, so the ring never paints the target's feet or anyone standing on it.
extends Decal

const ENTITY_LAYER := 2  # render layer bit of entity models; the decal leaves them out
const TEXTURE_SIZE := 256
const RING_AT := 0.8  # the ring's radius, as part of the decal's half width
const DEPTH := 3.0  # metres the decal reaches up and down, for slopes and stairs
const SPIN := 0.5  # radians per second
const PULSE_SPEED := 3.0  # radians per second
const MONSTER := Color(1.0, 0.25, 0.2)
const NPC := Color(0.35, 1.0, 0.4)
const PLAYER := Color(0.3, 0.6, 1.0)

static var _albedo: ImageTexture
static var _emission: ImageTexture

var _time := 0.0
var _target: Node3D
var _enemy_player := false


func _init() -> void:
	name = "TargetMarker"
	visible = false
	if _albedo == null:
		var ring := _draw_ring()
		_albedo = ImageTexture.create_from_image(ring)
		# Emission adds light wherever its colour isn't black, so it carries the ring's
		# brightness in its colour (premultiplied), not only in its alpha.
		ring.premultiply_alpha()
		_emission = ImageTexture.create_from_image(ring)
	texture_albedo = _albedo
	texture_emission = _emission
	emission_energy = 1.0
	albedo_mix = 1.0
	upper_fade = 0.3
	lower_fade = 0.3
	cull_mask = 0xFFFFF & ~ENTITY_LAYER
	top_level = true
	process_priority = 100  # after the entities (priority 0) have moved this frame


## Show the ring under target (a NetEntity), coloured for it; null hides it. Called by
## online.gd each frame; the ring moves in its own _process, after the entities have moved.
func follow(target: Node3D, enemy_player: bool) -> void:
	_target = target
	_enemy_player = enemy_player


func _process(delta: float) -> void:
	var target := _target
	if target == null or not is_instance_valid(target) or target.dead or target.dying or not target.visible:
		visible = false
		return
	var colour := PLAYER
	if target.is_npc:
		colour = NPC
	elif (target.is_monster and not target.is_summon) or _enemy_player:
		colour = MONSTER
	_time += delta
	var pulse := sin(_time * PULSE_SPEED)
	colour.a = 0.85 + 0.15 * pulse
	modulate = colour
	emission_energy = 1.1 + 0.4 * pulse
	var half: float = target.radius / RING_AT
	size = Vector3(half * 2.0, DEPTH, half * 2.0)
	global_position = target.global_position
	rotation = Vector3(0, fmod(_time * SPIN, TAU), 0)
	visible = true


## A white ring with a soft glow, a faint inner ring and four tick marks outside it,
## so the slow turn shows. Made once, while the zone loads.
static func _draw_ring() -> Image:
	var image := Image.create_empty(TEXTURE_SIZE, TEXTURE_SIZE, true, Image.FORMAT_RGBA8)
	var c := TEXTURE_SIZE * 0.5
	for y in TEXTURE_SIZE:
		for x in TEXTURE_SIZE:
			var dx := (x + 0.5 - c) / c
			var dy := (y + 0.5 - c) / c
			var r := sqrt(dx * dx + dy * dy)
			if r > 1.0:
				image.set_pixel(x, y, Color(1, 1, 1, 0))
				continue
			# The main ring: a sharp core with a glow either side.
			var d := absf(r - RING_AT)
			var a := clampf(1.0 - d / 0.025, 0.0, 1.0)
			a = maxf(a, 0.45 * exp(-d * d / 0.004))
			# A faint thin ring inside.
			a = maxf(a, 0.35 * clampf(1.0 - absf(r - 0.68) / 0.012, 0.0, 1.0))
			# Four ticks outside the ring.
			if r > 0.85 and r < 0.97:
				var angle := fposmod(atan2(dy, dx) + PI / 4.0, PI / 2.0) - PI / 4.0
				var across := absf(angle) * r
				a = maxf(a, clampf(1.0 - across / 0.03, 0.0, 1.0) * clampf((0.97 - r) / 0.03, 0.0, 1.0))
			image.set_pixel(x, y, Color(1, 1, 1, a))
	image.generate_mipmaps()
	return image
