#!/bin/bash

# Configuration
BASE_URL="http://127.0.0.1:3000"
ENDPOINT="/mosques/add-mosque-of-region"
URL="${BASE_URL}${ENDPOINT}"

# Regions to populate: "Name|South|West|North|East"
REGIONS=(
    "Jamia Hamdard, Delhi|28.51|77.24|28.53|77.26"
    "Mandawali, Delhi|28.61|77.28|28.64|77.31"
)

add_region() {
    local IFS='|'
    read -r name south west north east <<< "$1"
    
    local backoff=5
    local attempt=1
    
    echo "----------------------------------------------------"
    echo "Starting population for: $name"
    
    while true; do
        echo "[$(date +%T)] Attempt $attempt for $name..."
        
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

echo "Starting Mosque Population Script"
echo "Target URL: $URL"

for region in "${REGIONS[@]}"; do
    add_region "$region"
done

echo "----------------------------------------------------"
echo "All regions processed successfully!"
