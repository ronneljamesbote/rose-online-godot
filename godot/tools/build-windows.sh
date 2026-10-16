#!/bin/bash
# Builds the Windows release zip from Linux: cross-compiles rose_gd.dll with mingw-w64,
# exports ROSE.exe with Godot's Windows release template and zips both with the readme.
#
# Needs: rustup target x86_64-pc-windows-gnu, gcc-mingw-w64-x86-64, the Godot editor
# (GODOT, default godot on PATH) and its 4.7.2 export template windows_release_x86_64.exe
# in ~/.local/share/godot/export_templates/4.7.2.stable/, the Linux extension in
# godot/bin/librose_gd.so and the ROSE client data (ROSE_DATA_IDX, the path to data.idx).
# It refuses to build when overrides/motions.toml (the server's hit times) doesn't match
# the player animations; run --export-motions and commit the file first.
# Usage: ROSE_DATA_IDX=/path/to/data.idx godot/tools/build-windows.sh [OUT_DIR]
#        (OUT_DIR default: dist/ at the repository root)
set -euo pipefail
cd "$(dirname "$0")/../.."
GODOT=${GODOT:-godot}
OUT=$(realpath -m "${1:-dist}")
TARGET_DIR=${CARGO_TARGET_DIR:-$PWD/target}
DATA_IDX=${ROSE_DATA_IDX:?"set ROSE_DATA_IDX to the data.idx of the ROSE client"}

# The server times hits from overrides/motions.toml, so the animations shipped must match it.
"$GODOT" --headless --path godot -- --import-rose --data-idx="$DATA_IDX"
if ! "$GODOT" --headless --path godot -- --export-motions=check --data-idx="$DATA_IDX"; then
  echo "build-windows: overrides/motions.toml doesn't match the player animations." >&2
  echo "build-windows: run  godot --headless --path godot -- --export-motions  and commit it." >&2
  exit 1
fi
# The server places spawns, NPCs and named spots from overrides/zones/, exported from the zone scenes.
if ! "$GODOT" --headless --path godot -- --export-zones=check --data-idx="$DATA_IDX"; then
  echo "build-windows: overrides/zones/ doesn't match the zone scenes in godot/zones/." >&2
  echo "build-windows: run  godot --headless --path godot -- --export-zones  and commit it." >&2
  exit 1
fi

CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc CARGO_TARGET_DIR="$TARGET_DIR" \
  cargo build --release -p rose-gd --target x86_64-pc-windows-gnu
mkdir -p godot/bin
x86_64-w64-mingw32-strip -o godot/bin/rose_gd.dll "$TARGET_DIR/x86_64-pc-windows-gnu/release/rose_gd.dll"

STAGE=$(mktemp -d)
mkdir -p "$STAGE/ROSE"
"$GODOT" --headless --path godot --export-release "Windows Desktop" "$STAGE/ROSE/ROSE.exe"
cp godot/tools/README-windows.txt "$STAGE/ROSE/README.txt"
mkdir -p "$OUT"
rm -f "$OUT/ROSE-windows.zip"
(cd "$STAGE" && zip -qr "$OUT/ROSE-windows.zip" ROSE)
rm -rf "$STAGE"
ls -la "$OUT/ROSE-windows.zip"
