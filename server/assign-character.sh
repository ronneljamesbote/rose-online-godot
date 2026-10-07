#!/bin/sh
# Gives an existing character (one made before accounts) to an account from the website.
# Run inside the server container, with the account's email and password:
#   docker compose exec rose-server /opt/rose/assign-character.sh NAME EMAIL
# It asks for the password. The character must be offline and the account must not have
# made a character yet.
set -e
NAME=$1; EMAIL=$2
WEBSITE=${ROSE_AUTH_ISSUER:-http://rose-web:3001}
[ -n "$NAME" ] && [ -n "$EMAIL" ] || { echo "usage: assign-character.sh NAME EMAIL"; exit 1; }
printf "Password for %s: " "$EMAIL"
stty -echo 2>/dev/null || true
read -r PASSWORD
stty echo 2>/dev/null || true
echo
# JSON-escape backslashes and quotes in the password.
ESCAPED=$(printf '%s' "$PASSWORD" | sed 's/\\/\\\\/g; s/"/\\"/g')
ANSWER=$(curl -fsS -X POST -H 'content-type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"password\":\"$ESCAPED\"}" "$WEBSITE/api/game/login") || { echo "Signing in failed"; exit 1; }
TOKEN=$(printf '%s' "$ANSWER" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')
# SpacetimeDB tells us the identity this account plays as.
STDB=$(curl -fsS -X POST -H "Authorization: Bearer $TOKEN" http://127.0.0.1:3000/v1/identity/websocket-token)
PAYLOAD=$(printf '%s' "$STDB" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p' | cut -d. -f2 | tr '_-' '/+')
while [ $((${#PAYLOAD} % 4)) -ne 0 ]; do PAYLOAD="$PAYLOAD="; done
IDENTITY=$(printf '%s' "$PAYLOAD" | base64 -d 2>/dev/null | sed -n 's/.*"hex_identity":"\([0-9a-f]*\)".*/\1/p')
[ -n "$IDENTITY" ] || { echo "Could not find the account's identity"; exit 1; }
HOME=/data/cli spacetimedb-cli call --server http://127.0.0.1:3000 rose assign_character "\"$NAME\"" "0x$IDENTITY"
echo "$NAME now belongs to $EMAIL"
