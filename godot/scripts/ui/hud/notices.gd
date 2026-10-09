## Short messages (pickups, refused actions) as toasts that fade, newest at the bottom.
extends VBoxContainer

const SECONDS := 8.0
const MOST := 6


func _ready() -> void:
	mouse_filter = Control.MOUSE_FILTER_IGNORE
	add_theme_constant_override("separation", 4)
	alignment = BoxContainer.ALIGNMENT_END
	custom_minimum_size = Vector2(360, 0)


func add(text: String) -> void:
	var toast := PanelContainer.new()
	toast.theme_type_variation = "Tooltip"
	toast.mouse_filter = Control.MOUSE_FILTER_IGNORE
	toast.size_flags_horizontal = Control.SIZE_SHRINK_BEGIN
	var label := Label.new()
	label.text = text
	label.add_theme_color_override("font_color", get_theme_color("font_color", "TooltipLabel"))
	toast.add_child(label)
	add_child(toast)
	while get_child_count() > MOST:
		var oldest := get_child(0)
		remove_child(oldest)
		oldest.queue_free()
	var tween := toast.create_tween()
	toast.modulate.a = 0.0
	tween.tween_property(toast, "modulate:a", 1.0, 0.15)
	tween.tween_property(toast, "modulate:a", 0.0, 1.0).set_delay(SECONDS)
	tween.tween_callback(toast.queue_free)
