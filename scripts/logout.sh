#!/bin/bash
set -euo pipefail

# ============================================================
# logout.sh - Logout user
# ============================================================
# Usage:
#   ./scripts/logout.sh              # Uses saved token from .auth_state
#   ./scripts/logout.sh -t "token"   # Explicit token
#
# Logs out via the /auth/logout endpoint.
#
# Environment variables:
#   MERZAH_BASE_URL - Base URL of the server (default: http://127.0.0.1:3000)
# ============================================================

BASE_URL="${MERZAH_BASE_URL:-http://127.0.0.1:3000}"
ENDPOINT="/auth/logout"
URL="${BASE_URL}${ENDPOINT}"
STATE_FILE="$(dirname "$0")/.auth_state"

TOKEN=""

# Parse arguments
while getopts "t:" opt; do
    case $opt in
        t) TOKEN="$OPTARG" ;;
        *) echo "Usage: $0 [-t token]"; exit 1 ;;
    esac
done

# Load saved token if not provided
if [ -z "$TOKEN" ]; then
    if [ -f "$STATE_FILE" ]; then
        # shellcheck source=/dev/null
        source "$STATE_FILE"
        echo "Using saved token from $STATE_FILE"
    else
        echo "ERROR: No token provided and no saved state found."
        echo "Usage: $0 -t 'session_token'"
        exit 1
    fi
fi

if [ -z "${TOKEN:-}" ]; then
    echo "ERROR: No token available. Please login first."
    exit 1
fi

echo "===================================================="
echo "Logging out"
echo "===================================================="

# Logout uses DELETE method with Bearer token
RESPONSE=$(curl -s -w "\n%{http_code}" -X DELETE "$URL" \
    -H "Authorization: Bearer $TOKEN")

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo ""
echo "Response (HTTP $HTTP_CODE):"
echo "$BODY" | head -c 500
echo ""

if [ "$HTTP_CODE" -eq 200 ]; then
    echo ""
    echo "SUCCESS: Logged out!"

    # Clear token from state file (keep other fields)
    if [ -f "$STATE_FILE" ]; then
        sed -i 's/^TOKEN=.*/TOKEN=/' "$STATE_FILE"
        echo "Token cleared from $STATE_FILE"
    fi
else
    echo "WARNING: Logout returned HTTP $HTTP_CODE"
    echo "Response: $BODY"
    # Still clear the token locally
    if [ -f "$STATE_FILE" ]; then
        sed -i 's/^TOKEN=.*/TOKEN=/' "$STATE_FILE"
    fi
fi
