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

## Player characters

The import also builds the player characters from the converted meshes:

| File | What it is |
| --- | --- |
| `3ddata/avatar/male.tscn`, `female.tscn` | The skeleton (`Skeleton3D`, bones `b0`... and dummy bones `d0`...) and an `AnimationPlayer` with the no-weapon motions. |
| `3ddata/motion/avatar/*.anim` | Every player motion. Hit frames, footsteps and effect frames are markers on the timeline. |
| `3ddata/avatar/male_motions/weapon_N.tres` (and `female_motions`) | One animation library per weapon motion type (the column of `TYPE_MOTION.STB`): stop1, run, attack, attack2, attack3, hit, die... |
| `3ddata/avatar/list_mbody/12.tscn` and so on | One scene per part in each part list (faces, hair, caps, bodies, arms, feet; `3ddata/weapon/list_weapon` and `list_subwpn` for weapons and shields), named by the item's model number. Each mesh instance says whether it is `skinned` or which `bone` it hangs on. |

Markers are named `kind` + number + `:` + the ROSE event id, for example `hit1:21`. `hit`
markers are when a swing or skill lands (event ids 10, 20-28, 56, 57, 66, 67, the same ones
the server's `attack_hit_ms` uses), `step` markers play footsteps and `effect` markers the
other sounds and effects. Moving a marker in the Animation panel moves that event in game;
the id after the colon must stay.

The server times hits from `overrides/motions.toml`, so after moving a `hit` marker (or
changing an animation's length) export it and commit it with the animation:

```sh
godot --headless --path godot -- --export-motions          # writes ../overrides/motions.toml
godot --headless --path godot -- --export-motions=check    # fails when it is out of date
```

An edited animation goes in `ours/` under the same path (for example
`ours/3ddata/motion/avatar/onehand_attack01_m1.anim`), since `rose/` is remade by the import.

The game falls back to the client data for anything not in either folder, so it runs the
same with or without `rose/`. The Windows build leaves `rose/` out for now.
