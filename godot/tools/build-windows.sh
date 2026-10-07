#!/bin/bash
# Builds the Windows release zip from Linux: cross-compiles rose_gd.dll with mingw-w64,
# exports ROSE.exe with Godot's Windows release template and zips both with the readme.
#
# Needs: rustup target x86_64-pc-windows-gnu, gcc-mingw-w64-x86-64, the Godot editor
# (GODOT, default godot on PATH) and its 4.7.2 export template windows_release_x86_64.exe
# in ~/.local/share/godot/export_templates/4.7.2.stable/.
# Usage: godot/tools/build-windows.sh [OUT_DIR]   (default: dist/ at the repository root)
set -euo pipefail
cd "$(dirname "$0")/../.."
GODOT=${GODOT:-godot}
OUT=$(realpath -m "${1:-dist}")
TARGET_DIR=${CARGO_TARGET_DIR:-$PWD/target}

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
