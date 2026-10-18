@tool
class_name RoseTextureLoader
extends ResourceFormatLoader
## Opens the converted ROSE textures (`.rtex` in res://assets/rose/, written by
## --import-rose; see assets/README.md) as ImageTextures, in the game and in the editor.
##
## A `.rtex` is a compressed file holding "RTEX", the kind of payload (0 the client's DDS
## file, 1 a lossless WebP), whether to make the mipmaps here, a spare byte, then the
## payload. Mipmaps are made from the decompressed image, as the game does with the
## client's DDS files (texture.rs load_rose_image), so the pixels are the same.

const MAGIC := "RTEX"
const DDS := 0
const WEBP := 1


func _get_recognized_extensions() -> PackedStringArray:
	return PackedStringArray(["rtex"])


func _handles_type(type: StringName) -> bool:
	return ClassDB.is_parent_class("ImageTexture", type)


func _get_resource_type(path: String) -> String:
	return "ImageTexture" if path.get_extension().to_lower() == "rtex" else ""


## Having this makes Godot treat the files as having their own UIDs (none), so the editor
## doesn't write a .uid file next to each of them.
func _get_resource_uid(_path: String) -> int:
	return ResourceUID.INVALID_ID


func _load(path: String, _original_path: String, _use_sub_threads: bool, _cache_mode: int) -> Variant:
	var image := load_image(path)
	if image == null:
		return ERR_FILE_CORRUPT
	return ImageTexture.create_from_image(image)


## The image in a `.rtex`, with its mipmaps, or null.
static func load_image(path: String) -> Image:
	var file := FileAccess.open_compressed(path, FileAccess.READ, FileAccess.COMPRESSION_ZSTD)
	if file == null:
		push_error("rose: could not open %s" % path)
		return null
	var bytes := file.get_buffer(file.get_length())
	if bytes.size() < 7 or bytes.slice(0, 4).get_string_from_ascii() != MAGIC:
		push_error("rose: %s is not a converted ROSE texture" % path)
		return null
	var kind := bytes[4]
	var make_mipmaps := bytes[5] != 0
	var payload := bytes.slice(7)
	var image := Image.new()
	var error := image.load_dds_from_buffer(payload) if kind == DDS else image.load_webp_from_buffer(payload)
	if error != OK:
		push_error("rose: could not decode %s" % path)
		return null
	if make_mipmaps and not image.has_mipmaps():
		if image.is_compressed():
			image.decompress()
		image.generate_mipmaps()
	return image
