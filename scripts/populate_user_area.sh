#!/bin/bash

# Configuration
BASE_URL="http://127.0.0.1:3000"
ENDPOINT="/mosques/add-mosque-of-region"
URL="${BASE_URL}${ENDPOINT}"

# Load coordinates from .env (also optional USER_BOX_DELTA for box size in degrees)
set -a
source "$(dirname "$0")/../.env"
set +a

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
