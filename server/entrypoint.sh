#!/bin/sh
# Starts SpacetimeDB and installs (or updates) the rose module on every start.
# First start: makes the identity key pair.
# The game data (items, monsters, zones, ...) is uploaded from the ROSE client mounted at
# /game whenever the server has none or overrides/game-data.toml changed;
# ROSE_UPLOAD_GAME_DATA=1 uploads it again.
# ROSE_CLEAR_WORLD=1 wipes the world while publishing (needed after a breaking schema change).
# ROSE_AUTH_ISSUER is the account website the server trusts (empty: no accounts needed).
set -e
mkdir -p /data/keys /data/db /data/cli
if [ ! -f /data/keys/id_ecdsa.p8 ]; then
  echo "rose: first start, making the identity key pair in /data/keys"
  openssl ecparam -name prime256v1 -genkey -noout -out /data/keys/key.pem
  openssl pkcs8 -topk8 -nocrypt -in /data/keys/key.pem -out /data/keys/id_ecdsa.p8
  openssl ec -in /data/keys/key.pem -pubout -out /data/keys/id_ecdsa.pub 2>/dev/null
  rm /data/keys/key.pem
fi

spacetimedb-standalone start --listen-addr 0.0.0.0:3000 --data-dir /data/db \
  --jwt-pub-key-path /data/keys/id_ecdsa.pub --jwt-priv-key-path /data/keys/id_ecdsa.p8 \
  --non-interactive &
SERVER=$!
trap 'kill -TERM $SERVER; wait $SERVER' TERM INT

i=0
until curl -fs -o /dev/null http://127.0.0.1:3000/v1/ping; do
  i=$((i + 1))
  [ $i -gt 60 ] && { echo "rose: SpacetimeDB did not start"; exit 1; }
  sleep 1
done

# The CLI's login in /data/cli is the admin identity (it published the module).
CLEAR=""
[ "${ROSE_CLEAR_WORLD:-0}" = "1" ] && CLEAR="--delete-data=always" && echo "rose: ROSE_CLEAR_WORLD=1, wiping the world"
if HOME=/data/cli spacetimedb-cli publish --server http://127.0.0.1:3000 \
     --bin-path /opt/rose/rose_stdb_module.wasm -y $CLEAR rose; then
  echo "rose: module installed"
else
  echo "rose: installing the module failed (see above). If it reports a breaking schema change,"
  echo "rose: restart once with ROSE_CLEAR_WORLD=1 to start a fresh world."
fi
export HOME=/data/cli
# Accounts: trust game tokens from the account website (ROSE_AUTH_ISSUER, see compose.yaml).
# An empty value lets anyone in without an account.
if spacetimedb-cli call --server http://127.0.0.1:3000 rose set_auth_issuer "\"${ROSE_AUTH_ISSUER:-}\"" >/dev/null; then
  if [ -n "${ROSE_AUTH_ISSUER:-}" ]; then
    echo "rose: only players with an account from ${ROSE_AUTH_ISSUER} can sign in"
  else
    echo "rose: ROSE_AUTH_ISSUER is empty, anyone can connect without an account"
  fi
else
  echo "rose: setting the account website failed (see above)"
fi
READY=$(spacetimedb-cli sql --server http://127.0.0.1:3000 rose "SELECT ready FROM game_data_status" 2>/dev/null | grep -c true || true)
# A changed overrides file (built into the image) needs a new upload too.
export ROSE_OVERRIDES=/opt/rose/overrides/game-data.toml
OVERRIDES_SUM=$(sha256sum "$ROSE_OVERRIDES" | cut -d" " -f1)
if [ "$READY" != "0" ] && [ "$OVERRIDES_SUM" != "$(cat /data/overrides.sha256 2>/dev/null)" ]; then
  echo "rose: the game data overrides changed, uploading the game data again"
  ROSE_UPLOAD_GAME_DATA=1
fi
if [ "$READY" = "0" ] || [ "${ROSE_UPLOAD_GAME_DATA:-0}" = "1" ]; then
  DATA_IDX=$(find /game -maxdepth 2 -iname data.idx 2>/dev/null | head -n 1)
  if [ -n "$DATA_IDX" ]; then
    echo "rose: uploading the game data from $DATA_IDX"
    if /opt/rose/upload-game-data.sh "$DATA_IDX"; then
      echo "$OVERRIDES_SUM" > /data/overrides.sha256
    else
      echo "rose: the game data upload failed (see above)"
    fi
  else
    echo "rose: no data.idx under /game. Set ROSE_CLIENT to your ROSE client folder (see"
    echo "rose: compose.yaml) and restart; until then nobody can play."
  fi
fi
echo "rose: server ready on port 3000"
wait $SERVER
