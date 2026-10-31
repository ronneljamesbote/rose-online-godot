## The circle on the ground under the selected monster, NPC or player. One Decal, moved to
## the current target every frame, so it lies on the terrain, stairs and floors under the
## target's feet. Its size follows the target's model, its colour what the target is:
## red a monster (or a player we may fight in PvP), green an NPC, blue another player.
## A thick, see-through ring with a faint fill. It stands still; it pops in when something
## is selected and shrinks away when it is dropped.
##
## Characters, monsters and NPCs draw on render layer 2 (net_entity.gd), which the decal
## skips, so the circle never paints the target's feet or anyone standing on it.
extends Decal

const ENTITY_LAYER := 2  # render layer bit of entity models; the decal leaves them out
const TEXTURE_SIZE := 256
const RING_INNER := 0.72  # the ring's inner and outer edge, as parts of the decal's half width
const RING_OUTER := 0.94
const FILL := 0.18  # alpha of the fill inside the ring
const ALPHA := 0.75  # the whole circle is this see-through
const DEPTH := 3.0  # metres the decal reaches up and down, for slopes and stairs
const SELECT_SECONDS := 0.22  # the pop when something is selected
const DROP_SECONDS := 0.18  # the shrink when it is dropped
const MONSTER := Color(1.0, 0.22, 0.18)
const NPC := Color(0.3, 1.0, 0.35)
const PLAYER := Color(0.3, 0.6, 1.0)

static var _albedo: ImageTexture
static var _emission: ImageTexture

var _target: Node3D  # what online.gd has selected
var _enemy_player := false
var _shown: Node3D  # what the circle is on (or popping away from)
var _colour := PLAYER
var _half := 1.0
var _since := 0.0  # seconds since it popped in or started to go
var _dropping := false


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
	emission_energy = 0.6
	albedo_mix = 1.0
	upper_fade = 0.3
	lower_fade = 0.3
	cull_mask = 0xFFFFF & ~ENTITY_LAYER
	top_level = true
	process_priority = 100  # after the entities (priority 0) have moved this frame


## Show the circle under target (a NetEntity), coloured for it; null drops it. Called by
## online.gd each frame; the circle moves in its own _process, after the entities have moved.
func follow(target: Node3D, enemy_player: bool) -> void:
	_target = target
	_enemy_player = enemy_player


func _process(delta: float) -> void:
	var target := _target if _showable(_target) else null
	if target != null and target != _shown:
		# Something new is selected: pop in under it.
		_shown = target
		_dropping = false
		_since = 0.0
	elif target == null and _shown != null and not _dropping:
		_dropping = true  # dropped: shrink away where it was
		_since = 0.0
	if _shown == null:
		visible = false
		return
	_since += delta
	var scale_by := 1.0
	var alpha := 1.0
	if _dropping:
		var t := minf(_since / DROP_SECONDS, 1.0)
		scale_by = lerpf(1.0, 0.7, t)
		alpha = 1.0 - t
		if t >= 1.0:
			_shown = null
			visible = false
			return
	elif _since < SELECT_SECONDS:
		var t := _since / SELECT_SECONDS
		scale_by = lerpf(1.35, 1.0, 1.0 - (1.0 - t) * (1.0 - t))  # eases out
		alpha = minf(t * 2.0, 1.0)
	if not _dropping and is_instance_valid(_shown):
		_colour = PLAYER
		if _shown.is_npc:
			_colour = NPC
		elif (_shown.is_monster and not _shown.is_summon) or _enemy_player:
			_colour = MONSTER
		_half = _shown.radius / ((RING_INNER + RING_OUTER) * 0.5)
		global_position = _shown.global_position
	var colour := _colour
	colour.a = ALPHA * alpha
	modulate = colour
	var half := _half * scale_by
	size = Vector3(half * 2.0, DEPTH, half * 2.0)
	visible = true


func _showable(target: Node3D) -> bool:
	return target != null and is_instance_valid(target) and not target.dead and not target.dying and target.visible


## A white ring, solid between RING_INNER and RING_OUTER with soft edges, filled faintly
## inside. Made once, at sign in.
static func _draw_ring() -> Image:
	var image := Image.create_empty(TEXTURE_SIZE, TEXTURE_SIZE, true, Image.FORMAT_RGBA8)
	var c := TEXTURE_SIZE * 0.5
	var edge := 2.0 / TEXTURE_SIZE  # about two pixels of soft edge
	for y in TEXTURE_SIZE:
		for x in TEXTURE_SIZE:
			var dx := (x + 0.5 - c) / c
			var dy := (y + 0.5 - c) / c
			var r := sqrt(dx * dx + dy * dy)
			var ring := clampf((r - RING_INNER) / edge + 0.5, 0.0, 1.0) * clampf((RING_OUTER - r) / edge + 0.5, 0.0, 1.0)
			var inside := FILL * clampf((RING_INNER - r) / edge + 0.5, 0.0, 1.0)
			image.set_pixel(x, y, Color(1, 1, 1, maxf(ring, inside)))
	image.generate_mipmaps()
	return image
