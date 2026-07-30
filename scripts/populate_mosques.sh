#!/bin/bash
set -euo pipefail

# ============================================================
# populate_mosques.sh - Populate mosques for a geographic region
# ============================================================
# Usage:
#   ./scripts/populate_mosques.sh
#   ./scripts/populate_mosques.sh -s 28.51 -w 77.24 -n 28.53 -e 77.26
#
# Imports mosques from OpenStreetMap for a bounding box region.
# Requires app_admin or mosque_supervisor role.
#
# The add-mosque-of-region endpoint queries the Overpass API for
# places_of_worship with religion=muslim in the given bounding box.
#
# Environment variables:
#   MERZAH_BASE_URL  - Base URL of the server (default: http://127.0.0.1:3000)
#   MERZAH_TOKEN     - Session token (or loaded from .auth_state)
#   MERZAH_SOUTH     - South latitude (or use -s flag)
#   MERZAH_WEST      - West longitude (or use -w flag)
#   MERZAH_NORTH     - North latitude (or use -n flag)
#   MERZAH_EAST      - East longitude (or use -e flag)
# ============================================================

BASE_URL="${MERZAH_BASE_URL:-http://127.0.0.1:3000}"
ENDPOINT="/mosques/add-mosque-of-region"
URL="${BASE_URL}${ENDPOINT}"
STATE_FILE="$(dirname "$0")/.auth_state"

# Defaults: placeholder coordinates (Brooklyn, NY area)
SOUTH="${MERZAH_SOUTH:-40.57}"
WEST="${MERZAH_WEST:--73.99}"
NORTH="${MERZAH_NORTH:-40.73}"
EAST="${MERZAH_EAST:--73.85}"
TOKEN="${MERZAH_TOKEN:-}"

# Parse arguments
while getopts "s:w:n:e:t:" opt; do
    case $opt in
        s) SOUTH="$OPTARG" ;;
        w) WEST="$OPTARG" ;;
        n) NORTH="$OPTARG" ;;
        e) EAST="$OPTARG" ;;
        t) TOKEN="$OPTARG" ;;
        *) echo "Usage: $0 [-s south] [-w west] [-n north] [-e east] [-t token]"; exit 1 ;;
    esac
done

# Load saved token if not provided
if [ -z "$TOKEN" ]; then
    if [ -f "$STATE_FILE" ]; then
        # shellcheck source=/dev/null
        source "$STATE_FILE"
        echo "Using saved token from $STATE_FILE"
    fi
fi

if [ -z "${TOKEN:-}" ]; then
    echo "ERROR: No token available. Please login first."
    echo "Set MERZAH_TOKEN or run register.sh/login.sh first."
    exit 1
fi

echo "===================================================="
echo "Populating mosques"
echo "===================================================="
echo "  Bounding Box:"
echo "    South: $SOUTH"
echo "    West:  $WEST"
echo "    North: $NORTH"
echo "    East:  $EAST"
echo "===================================================="

# Retry with exponential backoff (same pattern as populate_delhi.sh)
backoff=5
attempt=1

while true; do
    echo "[$(date +%T)] Attempt $attempt..."

    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$URL" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN" \
        -d "{
            \"south\": $SOUTH,
            \"west\": $WEST,
            \"north\": $NORTH,
            \"east\": $EAST
        }")

    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')

    if [ "$HTTP_CODE" -eq 200 ]; then
        if echo "$BODY" | grep -q '"error":null' || ( ! echo "$BODY" | grep -q '"error":' && echo "$BODY" | grep -q '"data":' ); then
            echo "  SUCCESS: $BODY"
            echo ""
            echo "Mosques populated successfully!"
            break
        else
            echo "  FAILED: Server returned logic error: $BODY"
        fi
    elif [ "$HTTP_CODE" -eq 401 ] || [ "$HTTP_CODE" -eq 403 ]; then
        echo "  FAILED: HTTP $HTTP_CODE - Authorization required (need app_admin or mosque_supervisor role)"
        echo "  $BODY"
        exit 1
    elif [ "$HTTP_CODE" -eq 503 ] || [ "$HTTP_CODE" -eq 500 ]; then
        echo "  FAILED: HTTP $HTTP_CODE (OSM/Server Overload)"
    else
        echo "  FAILED: HTTP $HTTP_CODE - $BODY"
    fi

    echo "  Retrying in $backoff seconds..."
    sleep "$backoff"

    attempt=$((attempt + 1))
    backoff=$((backoff * 2))
    if [ "$backoff" -gt 120 ]; then
        backoff=120
    fi
done
