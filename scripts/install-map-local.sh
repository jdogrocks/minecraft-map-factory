#!/usr/bin/env bash
# Install a published Minecraft map to the local PaperMC Docker server.
#
# Usage: install-map-local.sh <published-map-dir> [<server-dir>]
#
# <published-map-dir>  path to output/published/<map-name>/ (the directory
#                      returned by the publisher, containing the world subdir)
# <server-dir>         path to the Minecraft server root
#                      (default: /home/jason/minecraft-server)

set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <published-map-dir> [<server-dir>]" >&2
    exit 1
fi

PUBLISHED_DIR="$(realpath "$1")"
SERVER_DIR="${2:-/home/jason/minecraft-server}"
COMPOSE_FILE="$SERVER_DIR/docker-compose.yml"
SERVICE_NAME="papermc"
# World data lives in the Docker volume mount (./data -> /data in container).
DATA_DIR="$SERVER_DIR/data"
SERVER_PROPERTIES="$DATA_DIR/server.properties"

if [[ ! -d "$PUBLISHED_DIR" ]]; then
    echo "Error: published map dir not found: $PUBLISHED_DIR" >&2
    exit 1
fi

if [[ ! -f "$COMPOSE_FILE" ]]; then
    echo "Error: docker-compose.yml not found at $COMPOSE_FILE" >&2
    exit 1
fi

# Find the world subdirectory — the single dir inside published-map-dir that
# contains a region/ folder.  After MIN-144 this is named after the geo area
# (e.g. Times_Square__NYC) rather than the old "MMF World N" scheme.
WORLD_SUBDIR=""
for d in "$PUBLISHED_DIR"/*/; do
    if [[ -d "${d}region" ]]; then
        WORLD_SUBDIR="$d"
        break
    fi
done

if [[ -z "$WORLD_SUBDIR" ]]; then
    echo "Error: no world subdir with region/ found in $PUBLISHED_DIR" >&2
    exit 1
fi

WORLD_NAME="$(basename "${WORLD_SUBDIR%/}")"

echo "==> Stopping $SERVICE_NAME..."
docker compose -f "$COMPOSE_FILE" stop "$SERVICE_NAME"

# Back up the existing world directory if present.
if [[ -d "$DATA_DIR/$WORLD_NAME" ]]; then
    BACKUP="$DATA_DIR/${WORLD_NAME}.bak.$(date +%Y%m%d_%H%M%S)"
    echo "==> Backing up existing world to $(basename "$BACKUP")..."
    mv "$DATA_DIR/$WORLD_NAME" "$BACKUP"
fi

echo "==> Copying $WORLD_NAME to $DATA_DIR/..."
cp -r "$WORLD_SUBDIR" "$DATA_DIR/$WORLD_NAME"

# Update level-name in server.properties so PaperMC loads the new world.
if [[ -f "$SERVER_PROPERTIES" ]]; then
    echo "==> Updating level-name=$WORLD_NAME in server.properties..."
    if grep -q "^level-name=" "$SERVER_PROPERTIES"; then
        sed -i "s/^level-name=.*/level-name=$WORLD_NAME/" "$SERVER_PROPERTIES"
    else
        echo "level-name=$WORLD_NAME" >> "$SERVER_PROPERTIES"
    fi
else
    echo "Warning: server.properties not found at $SERVER_PROPERTIES" >&2
fi

echo "==> Starting $SERVICE_NAME..."
docker compose -f "$COMPOSE_FILE" start "$SERVICE_NAME"

echo "==> Done!  World '$WORLD_NAME' installed.  Connect at localhost:25565."
