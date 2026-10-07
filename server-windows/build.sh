#!/bin/bash
# Builds dist/ROSE-server-windows.zip: SpacetimeDB for Windows, the game module, a fresh
# identity key pair and the start scripts. The zip holds a private key: give it only to
# whoever hosts the server.
# Usage: bash server-windows/build.sh [OUT_DIR]   (default: dist/)
set -euo pipefail
cd "$(dirname "$0")/.."
OUT=$(realpath -m "${1:-dist}")
TARGET_DIR=${CARGO_TARGET_DIR:-$PWD/module/target}
STDB_VERSION=2.10.2

(cd module && CARGO_TARGET_DIR="$TARGET_DIR" cargo build --release --target wasm32-unknown-unknown)

STAGE=$(mktemp -d)
mkdir -p "$STAGE/ROSE-server/keys"
curl -sSL -o "$STAGE/stdb.zip" \
  "https://github.com/clockworklabs/SpacetimeDB/releases/download/v$STDB_VERSION/spacetime-x86_64-pc-windows-msvc.zip"
unzip -q "$STAGE/stdb.zip" -d "$STAGE/ROSE-server"
cp "$TARGET_DIR/wasm32-unknown-unknown/release/rose_stdb_module.wasm" "$STAGE/ROSE-server/"
cp server-windows/start-server.bat server-windows/install-game.bat server-windows/README.txt "$STAGE/ROSE-server/"
openssl ecparam -name prime256v1 -genkey -noout -out "$STAGE/key.pem"
openssl pkcs8 -topk8 -nocrypt -in "$STAGE/key.pem" -out "$STAGE/ROSE-server/keys/id_ecdsa.p8"
openssl ec -in "$STAGE/key.pem" -pubout -out "$STAGE/ROSE-server/keys/id_ecdsa.pub" 2>/dev/null
mkdir -p "$OUT"
rm -f "$OUT/ROSE-server-windows.zip"
(cd "$STAGE" && zip -qr "$OUT/ROSE-server-windows.zip" ROSE-server)
rm -rf "$STAGE"
ls -la "$OUT/ROSE-server-windows.zip"
