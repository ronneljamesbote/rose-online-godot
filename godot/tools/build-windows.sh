#!/bin/bash
# Builds the Windows client from Linux: converts the ROSE models, textures and animations
# into godot/assets/rose/ (--import-rose), cross-compiles rose_gd.dll with mingw-w64,
# exports ROSE.exe and ROSE.pck (the scripts, the converted assets and the zone scenes)
# and zips them with the DLLs and the readme (in a folder ROSE, or ROSE-dev for --dev).
#
#   --dev     the dev client: GM commands in (chat and rose_gd.dll built with the dev
#             feature), Godot's debug template with a console window, the "Windows Desktop
#             Dev" preset. Output ROSE-windows-dev instead of ROSE-windows.
#   --folder  a folder instead of a zip (OUT_DIR/ROSE-windows[-dev]/), for testing. A rebuild
#             replaces only the game's own files, so the themes folder, the ROSE client
#             data and anything else put next to ROSE.exe stay.
#
# The production build checks that no GM code got in and stops if it did.
#
# Needs: rustup target x86_64-pc-windows-gnu, gcc-mingw-w64-x86-64, the Godot editor
# (GODOT, default godot on PATH), the Linux extension in godot/bin/librose_gd.so and the ROSE
# client data (ROSE_DATA_IDX, the path to data.idx). The editor's Windows export template
# (release, or debug and its console wrapper for --dev) is fetched into ~/.local/share/godot/export_templates/ when
# it's missing.
# It refuses to build when overrides/motions.toml (the server's hit times) doesn't match
# the player animations; run --export-motions and commit the file first.
# Usage: ROSE_DATA_IDX=/path/to/data.idx godot/tools/build-windows.sh [--dev] [--folder] [OUT_DIR]
#        (OUT_DIR default: dist/ at the repository root)
set -euo pipefail
cd "$(dirname "$0")/../.."
DEV=""
FOLDER=""
OUT_ARG=""
for arg in "$@"; do
  case "$arg" in
    --dev) DEV=1 ;;
    --folder) FOLDER=1 ;;
    -*) echo "build-windows: unknown option $arg" >&2; exit 2 ;;
    *) OUT_ARG=$arg ;;
  esac
done
GODOT=${GODOT:-godot}
OUT=$(realpath -m "${OUT_ARG:-dist}")
TARGET_DIR=${CARGO_TARGET_DIR:-$PWD/target}
DATA_IDX=${ROSE_DATA_IDX:?"set ROSE_DATA_IDX to the data.idx of the ROSE client"}
if [ -n "$DEV" ]; then
  NAME=ROSE-windows-dev DIR=ROSE-dev PRESET="Windows Desktop Dev" EXPORT=--export-debug
  # The console window comes from its own small template.
  TEMPLATE_FILES="windows_debug_x86_64.exe windows_debug_x86_64_console.exe"
else
  NAME=ROSE-windows DIR=ROSE PRESET="Windows Desktop" EXPORT=--export-release TEMPLATE_FILES=windows_release_x86_64.exe
fi

# The export template must match the editor's version; fetch it from the Godot release once.
GODOT_VERSION=$("$GODOT" --version | head -n1 | grep -oE '^[0-9]+\.[0-9]+(\.[0-9]+)?\.[a-z0-9]+')
TEMPLATES="${XDG_DATA_HOME:-$HOME/.local/share}/godot/export_templates/$GODOT_VERSION"
MISSING=""
for f in $TEMPLATE_FILES; do [ -f "$TEMPLATES/$f" ] || MISSING="$MISSING templates/$f"; done
if [ -n "$MISSING" ]; then
  RELEASE=${GODOT_VERSION%.*}-${GODOT_VERSION##*.}  # 4.7.2.stable -> 4.7.2-stable
  URL="https://github.com/godotengine/godot/releases/download/$RELEASE/Godot_v${RELEASE}_export_templates.tpz"
  echo "build-windows: fetching$MISSING from $URL"
  TPZ=$(mktemp --suffix=.tpz)
  curl -fL --retry 3 -o "$TPZ" "$URL"
  mkdir -p "$TEMPLATES"
  unzip -o -j "$TPZ" $MISSING -d "$TEMPLATES"
  rm -f "$TPZ"
fi

# The converted assets go into ROSE.pck, so make sure they are all there and current.
"$GODOT" --headless --path godot -- --import-rose --data-idx="$DATA_IDX"
# The server times hits from overrides/motions.toml, so the animations shipped must match it.
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
  cargo build --release -p rose-gd --target x86_64-pc-windows-gnu ${DEV:+--features dev}
mkdir -p godot/bin
x86_64-w64-mingw32-strip -o godot/bin/rose_gd.dll "$TARGET_DIR/x86_64-pc-windows-gnu/release/rose_gd.dll"

STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE" godot/build_version.txt' EXIT
mkdir -p "$STAGE/$DIR"
# The version line shows the commit the client was built from (scripts/build_info.gd).
git rev-parse --short=7 HEAD > godot/build_version.txt 2>/dev/null || echo unknown > godot/build_version.txt
# Godot keeps the binary copies of the text scenes it exported in .godot/exported/ and reuses
# one while its own file is unchanged, even when a scene it uses is new or changed. A zone
# scene exported before --import-rose made its model scenes would keep empty decorations and
# a grey ground, so convert everything again.
rm -rf godot/.godot/exported
"$GODOT" --headless --path godot "$EXPORT" "$PRESET" "$STAGE/$DIR/ROSE.exe"
cp godot/tools/README-windows.txt "$STAGE/$DIR/README.txt"

# GM commands are for dev builds only: the production client must have neither the GM chat
# scripts (godot/gm/) nor the extension's gm_command. A file in the pck is listed as
# "gm/gm_chat.gdc" and so on; the "res://gm/..." the pck also lists are only the project's
# uid cache naming every file, shipped or not.
if [ -z "$DEV" ]; then
  if LC_ALL=C grep -aoE '.{6}gm/gm_[a-z_]+\.gd' "$STAGE/$DIR/ROSE.pck" | LC_ALL=C grep -vq '^res://' \
    || grep -aq "gm_command" "$STAGE/$DIR/rose_gd.dll"; then
    echo "build-windows: GM code got into the production build (godot/gm/ or gm_command); not packaging it." >&2
    exit 1
  fi
fi

mkdir -p "$OUT"
if [ -n "$FOLDER" ]; then
  DEST="$OUT/$NAME"
  mkdir -p "$DEST"
  rm -f "$DEST"/*.exe "$DEST"/*.pck "$DEST"/*.dll "$DEST/README.txt"
  cp -r "$STAGE/$DIR/." "$DEST/"
  ls -la "$DEST"
else
  rm -f "$OUT/$NAME.zip"
  (cd "$STAGE" && zip -qr "$OUT/$NAME.zip" "$DIR")
  ls -la "$OUT/$NAME.zip"
fi
