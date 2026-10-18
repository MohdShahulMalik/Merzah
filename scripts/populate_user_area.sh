#!/bin/bash

# Configuration
BASE_URL="http://127.0.0.1:3000"
ENDPOINT="/mosques/add-mosque-of-region"
URL="${BASE_URL}${ENDPOINT}"

ENV_FILE="$(dirname "$0")/../.env"
AUTH_STATE_FILE="$(dirname "$0")/.auth_state"

# Read a single key from a dotenv-style file without executing it as a shell
# script. The files use "KEY = value" lines, so they cannot be sourced.
get_key() {
    local file="$1"
    local key="$2"
    local value
    value=$(grep -E "^[[:space:]]*${key}[[:space:]]*=" "$file" \
        | tail -n1 \
        | sed -E "s/^[^=]*=[[:space:]]*//")
    # Strip trailing carriage return, surrounding single/double quotes and inline comments
    value=$(printf '%s' "$value" | tr -d '\r')
    value=$(printf '%s' "$value" | sed -E "s/[[:space:]]+#.*$//")
    value=$(printf '%s' "$value" | sed -E "s/^\"(.*)\"$/\1/; s/^'(.*)'$/\1/")
    printf '%s' "$value"
}

USER_LAT=$(get_key "$ENV_FILE" USER_LAT)
USER_LON=$(get_key "$ENV_FILE" USER_LON)

AUTH_TOKEN=$(get_key "$AUTH_STATE_FILE" TOKEN)

if [ -z "$AUTH_TOKEN" ]; then
    echo "ERROR: TOKEN must be set in $AUTH_STATE_FILE"
    exit 1
fi

if [ -z "$USER_LAT" ] || [ -z "$USER_LON" ]; then
    echo "ERROR: USER_LAT and USER_LON must be set in .env"
    exit 1
fi

# Bounding box: half-width/half-height in degrees (default ~0.02 => roughly 2km)
DELTA="${USER_BOX_DELTA:-0.02}"

south=$(echo "$USER_LAT - $DELTA" | bc -l)
north=$(echo "$USER_LAT + $DELTA" | bc -l)
west=$(echo "$USER_LON - $DELTA" | bc -l)
east=$(echo "$USER_LON + $DELTA" | bc -l)

echo "Populating mosques around lat=$USER_LAT lon=$USER_LON"
echo "Bounding box: south=$south west=$west north=$north east=$east"

add_mosques() {
    local backoff=5
    local attempt=1

    echo "----------------------------------------------------"
    echo "Starting mosque population for ($south, $west, $north, $east)"

    while true; do
        echo "[$(date +%T)] Attempt $attempt..."

        # Send request using curl
        # Note: server_fn with Json codec expects a JSON object with arg names as keys
        response=$(curl -s -w "\n%{http_code}" -X POST "$URL" \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer ${AUTH_TOKEN}" \
            -d "{
                \"south\": $south,
                \"west\": $west,
                \"north\": $north,
                \"east\": $east
            }")

        http_code=$(echo "$response" | tail -n1)
        body=$(echo "$response" | sed '$d')

        if [ "$http_code" -eq 200 ]; then
            # Check if the ApiResponse contains an error
            # Robust check: Success if "error":null OR "error" is missing but "data" is present
            if echo "$body" | grep -q '"error":null' || ( ! echo "$body" | grep -q '"error":' && echo "$body" | grep -q '"data":' ); then
                echo "  SUCCESS: $body"
                return 0
            else
                echo "  FAILED: Server returned logic error: $body"
            fi
        elif [ "$http_code" -eq 503 ] || [ "$http_code" -eq 500 ]; then
            echo "  FAILED: HTTP $http_code (OSM/Server Overload)"
        elif [ "$http_code" -eq 401 ] || [ "$http_code" -eq 403 ]; then
            echo "  FAILED: HTTP $http_code (Authentication/authorization failed) - $body"
            echo "  Check the TOKEN in $AUTH_STATE_FILE. Not retrying."
            return 1
        else
            echo "  FAILED: HTTP $http_code - $body"
        fi

        echo "  Retrying in $backoff seconds..."
        sleep "$backoff"

        attempt=$((attempt + 1))
        # Exponential backoff up to 2 minutes
        backoff=$((backoff * 2))
        if [ "$backoff" -gt 120 ]; then
            backoff=120
        fi
    done
}

add_mosques

echo "----------------------------------------------------"
echo "Done!"
