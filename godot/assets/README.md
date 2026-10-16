# Game assets

Models and textures as Godot resources, in two folders with the same layout:

- `rose/` is made from the ROSE client data by the import command and is not committed.
- `ours/` holds edited and new art and is committed. A file here wins over the one with the
  same path in `rose/`.

A ROSE file's place in either folder is its client path in lower case with a Godot
extension: `3DDATA/AVATAR/MALE/BODY/BODY1_00.ZMS` is `3ddata/avatar/male/body/body1_00.mesh`
and `3DDATA/AVATAR/MALE/BODY/BODY1_00.DDS` is `3ddata/avatar/male/body/body1_00.res`. In
`ours/` a mesh may also be a `.res`, and a texture a `.png` or `.dds` (Godot imports those
when the project is opened in the editor).

## Making `rose/`

After building the extension (see `../README.md`), from the repository root:

```sh
godot --headless --path godot --import                     # once, so Godot knows the extension
godot --headless --path godot -- --import-rose --data-idx=/path/to/iRose_129_129/data.idx
```

It converts every ZMS mesh and DDS texture in `data.idx` except effects (`3DDATA/EFFECT`)
and the interface (`3DDATA/CONTROL`), which still load from the client data. Meshes keep
their bone weights; textures are saved as they decode, with the same mipmap fixes the game
applies at run time, so they look exactly as before. Running it again only converts what is
missing; `--import-rose=force` redoes everything. It also redoes everything after an update
that changes the converted output (`IMPORT_VERSION` in `rust/src/assets.rs`).

The game falls back to the client data for anything not in either folder, so it runs the
same with or without `rose/`. The Windows build leaves `rose/` out for now.
