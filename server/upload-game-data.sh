#!/bin/sh
# Uploads the game databases' source files from a ROSE client folder to the server.
# Usage: upload-game-data.sh <path to data.idx> [server URL] [database name]
# Needs rose-stdb-import (the import tool), spacetimedb-cli logged in as the admin
# (the identity that published the module) and curl. Applies the data changes in
# ROSE_OVERRIDES (default: overrides/game-data.toml in the current folder).
set -e
DATA_IDX=${1:?usage: upload-game-data.sh <data.idx> [server] [database]}
SERVER=${2:-http://127.0.0.1:3000}
DB=${3:-rose}
IMPORT=${ROSE_IMPORT:-rose-stdb-import}
CLI=${SPACETIME_CLI:-spacetimedb-cli}

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
"$IMPORT" pack "$DATA_IDX" "$TMP"
TOKEN=$("$CLI" login show --token 2>/dev/null | sed -n 's/^Your auth token.* is //p')
[ -n "$TOKEN" ] || { echo "upload-game-data: spacetimedb-cli is not logged in"; exit 1; }

call() {
  curl -fsS -X POST "$SERVER/v1/database/$DB/call/$1" \
    -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" --data-binary "@$2"
}
echo '[]' > "$TMP/none.json"
call begin_game_data_upload "$TMP/none.json"
for batch in "$TMP"/upload-*.json; do
  call upload_game_files "$batch"
done
call finish_game_data_upload "$TMP/none.json"
"$CLI" sql --server "$SERVER" "$DB" "SELECT version, files, bytes, ready, error FROM game_data_status"
