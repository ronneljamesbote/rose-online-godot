## Sound and visual effects: zone music (day and night tracks), ambient zone sounds, and
## what a character's animation frame events play (footsteps, swings, hits, arrows, skills).
## Ported from rose-offline-client's background_music_system, animation_sound_system and
## animation_effect_system. Volumes are kept in user://settings.cfg under [sound].
extends Node

const SETTINGS := "user://settings.cfg"
# AnimationEventFlags bits (rose-data animation_event_flags.rs).
const SOUND_FOOTSTEP := 1 << 0
const SOUND_WEAPON_ATTACK_START := 1 << 3
const SOUND_WEAPON_ATTACK_HIT := 1 << 4
const SOUND_WEAPON_FIRE_BULLET := 1 << 5
const SOUND_SKILL_FIRE_BULLET := 1 << 6
const SOUND_SKILL_DUMMY_HIT_0 := 1 << 7
const SOUND_SKILL_DUMMY_HIT_1 := 1 << 8
const SOUND_SKILL_HIT := 1 << 9
const EFFECT_SKILL_CASTING_0 := 1 << 12  # to 3: 1 << 15
const EFFECT_WEAPON_ATTACK_HIT := 1 << 16
const EFFECT_WEAPON_FIRE_BULLET := 1 << 17
const EFFECT_SKILL_FIRE_BULLET := 1 << 18
const EFFECT_SKILL_FIRE_DUMMY_BULLET := 1 << 19
const EFFECT_SKILL_DUMMY_HIT_0 := 1 << 20
const EFFECT_SKILL_DUMMY_HIT_1 := 1 << 21
const EFFECT_SKILL_HIT := 1 << 22
const EFFECT_SKILL_ACTION := 1 << 23
const LEVEL_UP_EFFECT := "3DDATA/EFFECT/LEVELUP_01.EFT"
const PROJECTILE_GRAVITY := 98.0  # parabola, as projectile_system.rs
# Loudness of each kind of sound (rose-offline-client sound_config.rs defaults).
const GAIN_MUSIC := 0.35
const GAIN_MY_FOOTSTEP := 0.9
const GAIN_OTHER_FOOTSTEP := 0.5
const GAIN_MY_COMBAT := 1.0
const GAIN_OTHER_COMBAT := 0.5
const GAIN_NPC := 0.6
const GAIN_UI := 0.5
const SOUND_RADIUS := 4.0  # metres at full volume
const MAX_DISTANCE := 60.0
const LEVEL_UP_SOUND := 16
const PICKUP_SOUND := 531

var camera: Camera3D
var online: Node  # online.gd, for looking up attack targets
var zone_id := 0
var log_sounds := false  # --sound-log: print each sound played
var music_volume := 1.0
var effects_volume := 1.0
var _listener: AudioListener3D
var _music: AudioStreamPlayer
var _music_night := false
var _ambient: Array[AudioStreamPlayer3D] = []
var _zone: Node
var _skills := {}  # skill id -> RoseFx.skill()
var _projectiles: Array[Dictionary] = []


func _ready() -> void:
	add_to_group("rose_fx")
	for bus in ["Music", "Effects"]:
		if AudioServer.get_bus_index(bus) < 0:
			AudioServer.add_bus()
			AudioServer.set_bus_name(AudioServer.bus_count - 1, bus)
	var settings := ConfigFile.new()
	settings.load(SETTINGS)
	set_volumes(settings.get_value("sound", "music", 1.0), settings.get_value("sound", "effects", 1.0), false)
	_listener = AudioListener3D.new()
	add_child(_listener)
	_music = AudioStreamPlayer.new()
	_music.bus = "Music"
	_music.volume_db = linear_to_db(GAIN_MUSIC)
	_music.finished.connect(_music.play)
	add_child(_music)


## The fx node of the scene, or null (scripts that play sounds look it up with this).
static func find(node: Node) -> Node:
	return node.get_tree().get_first_node_in_group("rose_fx") if node.is_inside_tree() else null


## Music and sound effect volumes, 0 to 1. Saved for the next start unless save is false.
func set_volumes(music: float, effects: float, save := true) -> void:
	music_volume = clampf(music, 0.0, 1.0)
	effects_volume = clampf(effects, 0.0, 1.0)
	_set_bus("Music", music_volume)
	_set_bus("Effects", effects_volume)
	if save:
		var settings := ConfigFile.new()
		settings.load(SETTINGS)
		settings.set_value("sound", "music", music_volume)
		settings.set_value("sound", "effects", effects_volume)
		settings.save(SETTINGS)


func _set_bus(bus: String, volume: float) -> void:
	var index := AudioServer.get_bus_index(bus)
	AudioServer.set_bus_volume_db(index, linear_to_db(maxf(volume, 0.0001)))
	AudioServer.set_bus_mute(index, volume <= 0.0)


## A new zone is loaded: its ambient sounds start and its music plays from the start.
func use_zone(zone: Node, id: int) -> void:
	zone_id = id
	_zone = zone
	for projectile in _projectiles:
		projectile["node"].queue_free()
	_projectiles.clear()
	for player in _ambient:
		player.queue_free()
	_ambient.clear()
	for sound in zone.get_ambient_sounds():
		var stream: AudioStream = RoseSound.stream(sound["path"])
		if stream == null:
			continue
		var player := AudioStreamPlayer3D.new()
		player.stream = stream
		player.bus = "Effects"
		player.unit_size = maxf(sound["range"], 1.0)
		player.max_distance = maxf(sound["range"] * 4.0, 20.0)
		player.volume_db = linear_to_db(GAIN_NPC)
		player.finished.connect(player.play)
		add_child(player)
		player.global_position = sound["position"]
		player.play(randf() * maxf(stream.get_length() - 0.1, 0.0))
		_ambient.append(player)
	_music.stop()
	_play_music()


## Called with the zone's time of day; switches between the day and night music.
func set_time_state(state: String) -> void:
	var night := state == "evening" or state == "night"
	if night != _music_night:
		_music_night = night
		_play_music()
	if _zone and is_instance_valid(_zone):
		_zone.set_night_effects(night)


func _play_music() -> void:
	var stream: AudioStream = RoseSound.zone_music(zone_id, _music_night)
	if stream == _music.stream and _music.playing:
		return
	_music.stream = stream
	if log_sounds:
		print("rose fx: %s music for zone %d %s" % ["night" if _music_night else "day", zone_id, "plays" if stream else "is missing"])
	if stream:
		_music.play()


func _process(delta: float) -> void:
	_move_projectiles(delta)
	# Like rose-offline-client: distances from our character, directions from the camera.
	if camera == null or not is_instance_valid(camera):
		return
	var at := camera.global_position
	var target = camera.get("target")
	if target is Node3D and is_instance_valid(target):
		at = target.global_position + Vector3(0, 1.5, 0)
	_listener.global_transform = Transform3D(camera.global_transform.basis, at)
	if not _listener.is_current():
		_listener.make_current()


## Plays a sound once at a position (or following a node), then frees the player.
func play_at(stream: AudioStream, where, gain := 1.0, what := "") -> void:
	if log_sounds and what != "":
		print("rose fx: %s %s" % [what, "plays" if stream else "has no sound"])
	if stream == null:
		return
	var player := AudioStreamPlayer3D.new()
	player.stream = stream
	player.bus = "Effects"
	player.unit_size = SOUND_RADIUS
	player.max_distance = MAX_DISTANCE
	player.volume_db = linear_to_db(gain)
	player.finished.connect(player.queue_free)
	if where is Node3D:
		where.add_child(player)
	else:
		add_child(player)
		player.global_position = where
	player.play()


## A sound for the interface (level up, pick up), not placed in the world.
func play_ui(stream: AudioStream, gain := GAIN_UI) -> void:
	if stream == null:
		return
	var player := AudioStreamPlayer.new()
	player.stream = stream
	player.bus = "Effects"
	player.volume_db = linear_to_db(gain)
	player.finished.connect(player.queue_free)
	add_child(player)
	player.play()


func level_up(entity: Node3D) -> void:
	spawn_effect(LEVEL_UP_EFFECT, entity)
	play_at(RoseSound.by_id(LEVEL_UP_SOUND), entity, GAIN_MY_COMBAT if entity.is_me else GAIN_OTHER_COMBAT, "level up")


func picked_up() -> void:
	play_ui(RoseSound.by_id(PICKUP_SOUND))


## A monster or NPC sound: "idle", "die" and so on (RoseSound.npc).
func npc_sound(entity: Node3D, kind: String) -> void:
	play_at(RoseSound.npc(entity.npc_id, kind), entity.global_position, GAIN_NPC, "%s sound of %s" % [kind, entity.label.text])


## A monster's death: its sound and its effect.
func npc_died(entity: Node3D) -> void:
	npc_sound(entity, "die")
	spawn_effect(RoseFx.npc_die(entity.npc_id), entity.global_position)


## Plays an EFT effect at a position, or riding on a node (a character or one of its bones).
## Returns the effect, or null if the file has nothing to show.
func spawn_effect(path: String, where) -> Node3D:
	if path == "" or where == null:
		return null
	var effect := RoseEffect.new()
	if not effect.load(path):
		effect.free()
		return null
	if log_sounds:
		print("rose fx: effect ", path)
	if where is Node3D:
		where.add_child(effect)
	else:
		get_parent().add_child(effect)
		effect.global_position = where
	return effect


## The node an effect rides on: a dummy bone of the character's skeleton, or the character.
func attach_point(entity: Node3D, bone: int) -> Node3D:
	if bone < 0 or entity.model == null:
		return entity
	var key := "FxBone%d" % bone
	var existing: Node = entity.model.find_child(key, true, false)
	if existing:
		return existing
	var skeleton: Skeleton3D = entity.model.find_child("Skeleton3D", true, false)
	if skeleton == null or skeleton.find_bone("d%d" % bone) < 0:
		return entity
	var attachment := BoneAttachment3D.new()
	attachment.name = key
	skeleton.add_child(attachment)
	attachment.bone_name = "d%d" % bone
	return attachment


## Where a projectile aims on a character: about its chest.
func _aim_point(entity: Node3D) -> Vector3:
	return entity.global_position + Vector3(0, minf(entity.height * 0.6, 3.0), 0)


## Sends a projectile (arrow, bullet, fireball) from a point to a character. `effect` is a
## RoseFx projectile dictionary; on arrival it shows its hit effect and sound, or the
## skill's hit effect.
func fire_projectile(from: Vector3, target: Node3D, effect: Dictionary, skill := {}) -> void:
	if effect.get("bullet", "") == "":
		return
	var node := Node3D.new()
	get_parent().add_child(node)
	node.global_position = from
	spawn_effect(effect["bullet"], node)
	_projectiles.append({"node": node, "target": target, "effect": effect, "skill": skill, "parabola": {}})


func _move_projectiles(delta: float) -> void:
	for i in range(_projectiles.size() - 1, -1, -1):
		var p: Dictionary = _projectiles[i]
		var node: Node3D = p["node"]
		var target: Node3D = p["target"]
		if not is_instance_valid(node) or not is_instance_valid(target):
			if is_instance_valid(node):
				node.queue_free()
			_projectiles.remove_at(i)
			continue
		var to := _aim_point(target)
		var effect: Dictionary = p["effect"]
		var speed: float = maxf(effect.get("speed", 10.0), 1.0)
		var step := Vector3.ZERO
		var arrived := false
		match effect.get("move", "linear"):
			"immediate":
				arrived = true
			"parabola":
				var parabola: Dictionary = p["parabola"]
				if parabola.is_empty():
					var travel := node.global_position.distance_to(to) / speed
					var move := (to - node.global_position).normalized() * speed
					parabola.merge({"start_y": node.global_position.y, "end_y": to.y, "vy": travel * PROJECTILE_GRAVITY / 2.0,
						"move": move, "time": 0.0, "total": maxf(travel, 0.01)})
				parabola["vy"] -= PROJECTILE_GRAVITY * delta
				parabola["time"] += delta
				var move: Vector3 = parabola["move"]
				move.y = parabola["vy"]
				step = move * delta
				step.y += (parabola["end_y"] - parabola["start_y"]) / parabola["total"] * delta
				arrived = parabola["time"] >= parabola["total"]
			_:
				var distance := node.global_position.distance_to(to)
				step = (to - node.global_position).normalized() * speed * delta
				arrived = step.length() + 0.1 >= distance
		if arrived:
			_projectile_hit(target, effect, p["skill"])
			node.queue_free()
			_projectiles.remove_at(i)
			continue
		node.global_position += step
		if step.length() > 0.0001 and absf(step.normalized().y) < 0.999:
			# Projectile effects point along +X.
			node.global_basis = Basis.looking_at(step.normalized(), Vector3.UP) * Basis(Vector3.UP, PI / 2.0)


func _projectile_hit(target: Node3D, effect: Dictionary, skill: Dictionary) -> void:
	var gain := GAIN_MY_COMBAT if target.is_me else GAIN_OTHER_COMBAT
	play_at(effect.get("hit_sound"), target.global_position, gain, "projectile hit on " + target.label.text)
	if not skill.is_empty() and skill.get("hit", "") != "":
		spawn_effect(skill["hit"], attach_point(target, skill.get("hit_bone", -1)))
	else:
		spawn_effect(effect.get("hit", ""), target.global_position)


func _skill(id: int) -> Dictionary:
	if not _skills.has(id):
		_skills[id] = RoseFx.skill(id)
	return _skills[id]


## One animation frame event of a character: plays what its flags ask for.
func frame_event(entity: Node3D, event: int) -> void:
	var flags: int = RoseSound.event_flags(event)
	if flags == 0:
		return
	var skill: int = entity.active_skill()
	var target: Node3D = _target_of(entity, skill > 0)
	var mine: bool = entity.is_me or (target != null and target.is_me)
	var combat_gain := GAIN_MY_COMBAT if mine else GAIN_OTHER_COMBAT
	var weapon := _weapon_of(entity)

	if flags & SOUND_FOOTSTEP:
		var tile := -1
		var zone: Node = entity.zone
		if zone and entity.global_position.y <= zone.get_terrain_height(entity.global_position.x, entity.global_position.z) + 0.05:
			tile = zone.get_tile_index(entity.global_position.x, entity.global_position.z)
		play_at(RoseSound.footstep(zone_id, tile), entity.global_position, GAIN_MY_FOOTSTEP if entity.is_me else GAIN_OTHER_FOOTSTEP, "footstep of %s on tile %d" % [entity.label.text, tile])
	if skill > 0:
		var data := _skill(skill)
		for i in 4:
			if flags & (EFFECT_SKILL_CASTING_0 << i):
				var casting: Dictionary = data.get("casting", [{}, {}, {}, {}])[i]
				spawn_effect(casting.get("path", ""), attach_point(entity, casting.get("bone", -1)))
		if flags & EFFECT_SKILL_ACTION and data.get("type", "") == "self":
			spawn_effect(data.get("self_effect", ""), attach_point(entity, data.get("bullet_bone", -1)))
			spawn_effect(data.get("hit", ""), attach_point(entity, data.get("hit_bone", -1)))
	if flags & SOUND_WEAPON_ATTACK_START:
		play_at(RoseSound.attack_start(weapon, entity.npc_id if entity.is_monster else 0), entity.global_position, combat_gain, "swing of " + entity.label.text)
	if target == null:
		return
	if flags & EFFECT_WEAPON_ATTACK_HIT:
		var hit: Dictionary = RoseFx.weapon_hit(weapon, entity.npc_id if entity.is_monster else 0)
		spawn_effect(hit.get("hit", ""), target.global_position)
	if flags & EFFECT_WEAPON_FIRE_BULLET:
		fire_projectile(_aim_point(entity), target, RoseFx.weapon_projectile(weapon))
	if skill > 0:
		var data := _skill(skill)
		var bullet: Dictionary = data.get("bullet", {})
		var fires := flags & (EFFECT_SKILL_FIRE_BULLET | EFFECT_SKILL_FIRE_DUMMY_BULLET) != 0
		if flags & EFFECT_SKILL_ACTION and data.get("type", "") in ["bullet", "target"]:
			fires = true
		if fires:
			var from := attach_point(entity, data.get("bullet_bone", -1)).global_position
			if from == entity.global_position:
				from = _aim_point(entity)
			fire_projectile(from, target, bullet, data)
		if flags & EFFECT_SKILL_HIT:
			if data.get("hit", "") != "":
				spawn_effect(data["hit"], attach_point(target, data.get("hit_bone", -1)))
			else:
				spawn_effect(RoseFx.weapon_hit(weapon, entity.npc_id if entity.is_monster else 0).get("hit", ""), target.global_position)
		for i in 2:
			if flags & (EFFECT_SKILL_DUMMY_HIT_0 << i):
				spawn_effect(data.get("dummy_hits", PackedStringArray(["", ""]))[i], target)
	if flags & SOUND_WEAPON_ATTACK_HIT:
		play_at(RoseSound.attack_hit(weapon, target.npc_id if target.is_monster else 0), target.global_position, combat_gain, "hit by %s on %s" % [entity.label.text, target.label.text])
	if flags & SOUND_WEAPON_FIRE_BULLET:
		play_at(RoseSound.weapon_fire(weapon), entity.global_position, combat_gain, "shot of " + entity.label.text)
	if skill > 0:
		if flags & SOUND_SKILL_FIRE_BULLET:
			play_at(RoseSound.skill(skill, "fire"), entity.global_position, combat_gain, "skill %d fire sound" % skill)
		if flags & SOUND_SKILL_HIT:
			play_at(RoseSound.skill(skill, "hit"), target.global_position, combat_gain, "skill %d hit sound" % skill)
		if flags & SOUND_SKILL_DUMMY_HIT_0:
			play_at(RoseSound.skill(skill, "dummy0"), target.global_position, combat_gain, "skill %d dummy0 sound" % skill)
		if flags & SOUND_SKILL_DUMMY_HIT_1:
			play_at(RoseSound.skill(skill, "dummy1"), target.global_position, combat_gain, "skill %d dummy1 sound" % skill)


## What the entity hits: its skill's target while a skill motion plays, else its attack target.
## The server applies a skill when its action starts and a swing at its hit frame, so a
## target they kill is already gone from the zone when the bolt or arrow flies; the node
## kept from before still plays its death.
func _target_of(entity: Node3D, casting := false) -> Node3D:
	if casting and entity.cast_target >= 0:
		var kept: Node3D = entity.cast_target_node
		return kept if kept != null and is_instance_valid(kept) else entity_by_id(entity.cast_target)
	var target := entity_by_id(entity.target_id)
	if target == null and entity.target_id < 0:
		# The same for a swing whose hit killed: the swing still shows its arrow and spark.
		var last: Node3D = entity.last_target_node
		if last != null and is_instance_valid(last):
			return last
	return target


## The entity with this id in our zone, or null.
func entity_by_id(id: int) -> Node3D:
	if online == null or id < 0:
		return null
	var target = online.entities.get(id)
	return target if target != null and is_instance_valid(target) else null


func _weapon_of(entity: Node3D) -> int:
	if entity.is_monster or entity.is_npc or entity.look.size() < 8:
		return 0
	return int(entity.look[7])
