#!/bin/sh
# Starts SpacetimeDB and installs (or updates) the rose module on every start.
# First start: makes the identity key pair; the module's init seeds Zant's monsters.
# ROSE_CLEAR_WORLD=1 wipes the world while publishing (needed after a breaking schema change).
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
  echo "rose: server ready on port 3000"
else
  echo "rose: installing the module failed (see above). If it reports a breaking schema change,"
  echo "rose: restart once with ROSE_CLEAR_WORLD=1 to start a fresh world."
fi
wait $SERVER
