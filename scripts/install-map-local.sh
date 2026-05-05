#!/usr/bin/env bash
# Install a published pipeline map into the local Minecraft server (Docker).
#
# Usage:
#   ./scripts/install-map-local.sh <map-name>   install a specific published map
#   ./scripts/install-map-local.sh --latest     install the most-recently published map
#   ./scripts/install-map-local.sh --list       list available published maps

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PUBLISHED_DIR="$REPO_ROOT/pipeline/output/published"
MC_SERVER_DIR="/home/jason/minecraft-server"
MC_DATA_DIR="$MC_SERVER_DIR/data"
SERVER_PROPS="$MC_DATA_DIR/server.properties"
COMPOSE_FILE="$MC_SERVER_DIR/docker-compose.yml"
WORLD_SUBDIR="MMF World 1"

usage() {
    echo "Usage: $0 <map-name> | --latest | --list" >&2
    echo "" >&2
    echo "Options:" >&2
    echo "  <map-name>  Install a specific published map by directory name" >&2
    echo "  --latest    Install the most-recently modified published map" >&2
    echo "  --list      List available published maps and exit" >&2
    exit 1
}

list_maps() {
    if [[ ! -d "$PUBLISHED_DIR" ]]; then
        echo "No published maps directory found at $PUBLISHED_DIR" >&2
        exit 1
    fi
    echo "Available published maps:"
    ls -1t "$PUBLISHED_DIR" | sed 's/^/  /'
}

[[ $# -lt 1 ]] && usage

case "$1" in
    --list)
        list_maps
        exit 0
        ;;
    --latest)
        MAP_NAME=$(ls -1t "$PUBLISHED_DIR" 2>/dev/null | head -1)
        if [[ -z "$MAP_NAME" ]]; then
            echo "No published maps found in $PUBLISHED_DIR" >&2
            exit 1
        fi
        echo "==> Latest map: $MAP_NAME"
        ;;
    --*)
        usage
        ;;
    *)
        MAP_NAME="$1"
        ;;
esac

MAP_SRC="$PUBLISHED_DIR/$MAP_NAME/$WORLD_SUBDIR"
if [[ ! -d "$MAP_SRC" ]]; then
    echo "Map not found: $MAP_SRC" >&2
    echo "Run '$0 --list' to see available maps." >&2
    exit 1
fi

# Minecraft world directory names cannot contain slashes; spaces work but
# are inconvenient. Use the pipeline map name as-is (it is already
# underscore-sanitized by the publisher).
DEST_NAME="$MAP_NAME"
DEST_WORLD="$MC_DATA_DIR/$DEST_NAME"

if [[ ! -f "$COMPOSE_FILE" ]]; then
    echo "Docker compose file not found: $COMPOSE_FILE" >&2
    exit 1
fi

echo "==> Installing '$MAP_NAME' → $DEST_WORLD"

echo "==> Stopping Minecraft server"
docker compose -f "$COMPOSE_FILE" stop papermc

# Back up any existing installation of this map
if [[ -d "$DEST_WORLD" ]]; then
    BACKUP="${DEST_WORLD}.bak.$(date +%Y%m%d-%H%M%S)"
    echo "==> Backing up existing world to $(basename "$BACKUP")"
    mv "$DEST_WORLD" "$BACKUP"
fi

echo "==> Copying map files"
cp -r "$MAP_SRC" "$DEST_WORLD"

# Update level-name in server.properties so this world loads on startup.
# The itzg Docker image only overrides level-name when the LEVEL env var is
# set in docker-compose.yml; since we don't set it, server.properties is
# authoritative.
if [[ -f "$SERVER_PROPS" ]]; then
    echo "==> Setting level-name=$DEST_NAME in server.properties"
    # Use a temp file to avoid partial writes on sed -i
    TMP=$(mktemp)
    sed "s/^level-name=.*/level-name=$DEST_NAME/" "$SERVER_PROPS" > "$TMP"
    mv "$TMP" "$SERVER_PROPS"
else
    echo "WARNING: server.properties not found; you will need to set level-name=$DEST_NAME manually." >&2
fi

echo "==> Starting Minecraft server"
docker compose -f "$COMPOSE_FILE" start papermc

echo ""
echo "Done. '$MAP_NAME' is now the active world."
echo "Connect at: localhost:25565"
