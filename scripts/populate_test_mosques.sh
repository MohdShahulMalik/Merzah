#!/bin/bash
set -euo pipefail

# ============================================================
# populate_test_mosques.sh - Add fake test mosques and their events
# ============================================================
# Usage:
#   ./scripts/populate_test_mosques.sh
#   ./scripts/populate_test_mosques.sh -c 7 -t "session_token"
#
# Adds fake (test) mosques around the user's coordinates and attaches a fake
# event to every one of them, using nothing but the server functions of the
# app (curl calls only). It doubles as an integration test: every call is
# checked and the script exits with a non zero status code when something
# does not behave as expected.
#
# The mosques are added through the "add-mosque" server function, which only
# an app admin is allowed to call. Run ./scripts/make_app_admin.sh first if
# the user of the saved session is not an app admin yet.
#
# Every run creates new mosques: their names carry a run id and the server
# gives every mosque its own record id.
#
# Environment variables:
#   MERZAH_BASE_URL  - Base URL of the server (default: http://127.0.0.1:3000)
#   MERZAH_TOKEN     - Session token (or loaded from .auth_state)
#   MERZAH_USER_LAT  - Latitude of the user (or USER_LAT from .env)
#   MERZAH_USER_LON  - Longitude of the user (or USER_LON from .env)
# ============================================================

BASE_URL="${MERZAH_BASE_URL:-http://127.0.0.1:3000}"
SCRIPT_DIR="$(dirname "$0")"
STATE_FILE="${SCRIPT_DIR}/.auth_state"
ENV_FILE="${SCRIPT_DIR}/../.env"

TOKEN="${MERZAH_TOKEN:-}"
COUNT=7
RADIUS_KM=5

# Read a single key from a dotenv-style file without executing it as a shell
# script. The files use "KEY = value" lines, so they cannot be sourced.
get_key() {
    local file="$1"
    local key="$2"
    local value
    value=$(grep -E "^[[:space:]]*${key}[[:space:]]*=" "$file" \
        | tail -n1 \
        | sed -E "s/^[^=]*=[[:space:]]*//")
    value=$(printf '%s' "$value" | tr -d '\r')
    value=$(printf '%s' "$value" | sed -E "s/[[:space:]]+#.*$//")
    value=$(printf '%s' "$value" | sed -E "s/^\"(.*)\"$/\1/; s/^'(.*)'$/\1/")
    printf '%s' "$value"
}

# Parse arguments
while getopts "c:t:r:" opt; do
    case $opt in
        c) COUNT="$OPTARG" ;;
        t) TOKEN="$OPTARG" ;;
        r) RADIUS_KM="$OPTARG" ;;
        *) echo "Usage: $0 [-c count] [-t token] [-r radius_km]"; exit 1 ;;
    esac
done

# Load saved token if not provided
if [ -z "$TOKEN" ]; then
    if [ -f "$STATE_FILE" ]; then
        # shellcheck source=/dev/null
        source "$STATE_FILE"
        TOKEN="${TOKEN:-}"
        echo "Using saved token from $STATE_FILE"
    fi
fi

if [ -z "${TOKEN:-}" ]; then
    echo "ERROR: No token available. Please login first."
    echo "Set MERZAH_TOKEN or run register.sh/login.sh first."
    exit 1
fi

USER_LAT="${MERZAH_USER_LAT:-}"
USER_LON="${MERZAH_USER_LON:-}"

if [ -z "$USER_LAT" ] || [ -z "$USER_LON" ]; then
    USER_LAT=$(get_key "$ENV_FILE" USER_LAT)
    USER_LON=$(get_key "$ENV_FILE" USER_LON)
fi

if [ -z "$USER_LAT" ] || [ -z "$USER_LON" ]; then
    echo "ERROR: USER_LAT and USER_LON must be set in .env (or MERZAH_USER_LAT/MERZAH_USER_LON)"
    exit 1
fi

# ============================================================
# Helpers
# ============================================================

HTTP_CODE=""
RESPONSE_BODY=""

# Performs a POST request against a server function and stores the status
# code in HTTP_CODE and the body in RESPONSE_BODY.
api_post() {
    local path="$1"
    local body="$2"
    local response

    response=$(curl -s -w "\n%{http_code}" -X POST "${BASE_URL}${path}" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer ${TOKEN}" \
        -d "$body")

    HTTP_CODE=$(printf '%s' "$response" | tail -n1)
    RESPONSE_BODY=$(printf '%s' "$response" | sed '$d')
}

# Performs a GET request against a server function and stores the status
# code in HTTP_CODE and the body in RESPONSE_BODY.
api_get() {
    local path="$1"
    local response

    response=$(curl -s -w "\n%{http_code}" -X GET "${BASE_URL}${path}" \
        -H "Authorization: Bearer ${TOKEN}")

    HTTP_CODE=$(printf '%s' "$response" | tail -n1)
    RESPONSE_BODY=$(printf '%s' "$response" | sed '$d')
}

# Extracts the "data" string of an ApiResponse.
extract_data_string() {
    printf '%s' "$1" \
        | grep -o '"data":"[^"]*"' \
        | head -n1 \
        | sed -e 's/"data":"//' -e 's/"$//'
}

# Returns 0 when the body carries no error.
has_no_error() {
    printf '%s' "$1" | grep -q '"error":null' \
        || ( ! printf '%s' "$1" | grep -q '"error":' && printf '%s' "$1" | grep -q '"data":' )
}

# Computes a random point around the user's coordinates. The distance from
# the center is kept below the radius, the direction is spread over the whole
# circle so that the mosques do not end up on top of each other.
random_point() {
    local index="$1"
    local count="$2"
    local seed="$3"

    awk -v idx="$index" -v count="$count" -v lat="$USER_LAT" -v lon="$USER_LON" \
        -v seed="$seed" -v radius="$RADIUS_KM" 'BEGIN {
        srand(seed);
        pi = atan2(0, -1);
        # spread the mosques evenly over the circle and jitter them a bit
        angle = 2 * pi * (idx - 1) / count + (rand() - 0.5) * 0.8;
        # keep some distance from the exact center and stay inside the radius
        distance = (0.3 + rand() * 0.6) * radius;
        rad = pi / 180;
        dlat = (distance * cos(angle)) / 111.32;
        dlon = (distance * sin(angle)) / (111.32 * cos(lat * rad));
        printf "%.6f %.6f", lat + dlat, lon + dlon;
    }'
}

# Distance in kilometers between two points, used for the report.
distance_km() {
    awk -v lat1="$1" -v lon1="$2" -v lat2="$3" -v lon2="$4" 'BEGIN {
        pi = atan2(0, -1);
        rad = pi / 180;
        dlat = (lat2 - lat1) * rad;
        dlon = (lon2 - lon1) * rad;
        a = sin(dlat / 2) * sin(dlat / 2) + cos(lat1 * rad) * cos(lat2 * rad) * sin(dlon / 2) * sin(dlon / 2);
        printf "%.2f", 6371 * 2 * atan2(sqrt(a), sqrt(1 - a));
    }'
}

# ============================================================
# Test data
# ============================================================

RUN_ID="$(date +%s)-$$-${RANDOM}"

MOSQUE_NAMES=(
    "Masjid An-Nur"
    "Al-Hikmah Mosque"
    "Masjid Al-Iman"
    "Darul Uloom Mosque"
    "Masjid As-Salam"
    "Al-Falah Mosque"
    "Masjid Al-Amin"
    "Masjid Al-Baqiyat"
    "Masjid Al-Furqan"
    "Masjid At-Taqwa"
)

EVENT_TITLES=(
    "Friday Night Halaqah"
    "Study Circle on Surah Al-Kahf"
    "Community Iftar Gathering"
    "Youth Quran Circle"
    "Practical Parenting Workshop"
    "Weekly Tafsir Class"
    "Neighborhood Lecture"
)

EVENT_CATEGORIES=("halaqah" "lecture" "community" "youth" "workshop" "taraweeh" "seminar")

EVENT_DESCRIPTIONS=(
    "A fake event created by populate_test_mosques.sh to exercise the event endpoints of the app."
    "A fake event created by populate_test_mosques.sh so that the mosque shows up with a hosted event."
    "A fake event created by populate_test_mosques.sh, used as test data for the event listings."
)

SPEAKERS=("Sheikh Test" "Ustadh Test" "Imam Test" "")

echo "===================================================="
echo "Adding fake test mosques"
echo "===================================================="
echo "  Base URL:      $BASE_URL"
echo "  User coords:   lat=$USER_LAT lon=$USER_LON"
echo "  Radius:        ${RADIUS_KM} km"
echo "  Mosques:       $COUNT"
echo "  Run id:        $RUN_ID"
echo "===================================================="
echo ""

FAILURES=0
MOSQUE_IDS=()
MOSQUE_LABELS=()

# ============================================================
# Step 1: add the mosques
# ============================================================
echo "[1/4] Adding $COUNT mosques..."

for i in $(seq 1 "$COUNT"); do
    base_name="${MOSQUE_NAMES[$(( (i - 1) % ${#MOSQUE_NAMES[@]} ))]}"
    name="${base_name} ${RUN_ID}-${i}"

    read -r lat lon <<< "$(random_point "$i" "$COUNT" "${RANDOM}${i}${RUN_ID}")"

    if [ -z "$lat" ] || [ -z "$lon" ]; then
        echo "  FAILED: could not compute a location for the mosque"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    payload=$(cat <<JSON
{
    "mosque": {
        "name": "$name",
        "lat": $lat,
        "lon": $lon,
        "street": "Test Street $i",
        "city": "Test City",
        "cover_img": null
    }
}
JSON
)

    api_post "/mosques/add-mosque" "$payload"

    if [ "$HTTP_CODE" = "401" ] || [ "$HTTP_CODE" = "403" ]; then
        echo "  FAILED: HTTP $HTTP_CODE - only an app admin can approve the addition of a mosque"
        echo "  Run ./scripts/make_app_admin.sh and try again."
        exit 1
    fi

    if [ "$HTTP_CODE" != "200" ] && [ "$HTTP_CODE" != "201" ]; then
        echo "  FAILED: HTTP $HTTP_CODE - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    if ! has_no_error "$RESPONSE_BODY"; then
        echo "  FAILED: $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    mosque_id=$(extract_data_string "$RESPONSE_BODY")

    if [ -z "$mosque_id" ]; then
        echo "  FAILED: could not read the id of the mosque from $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    MOSQUE_IDS+=("$mosque_id")
    MOSQUE_LABELS+=("$name")

    echo "  OK: $name -> $mosque_id ($(distance_km "$USER_LAT" "$USER_LON" "$lat" "$lon") km away)"
done

echo ""

# ============================================================
# Step 2: attach an event to every mosque
# ============================================================
echo "[2/4] Adding an event to every mosque..."

EVENT_DATE=$(date -u -d "+2 days" +"%Y-%m-%dT18:00:00+00:00" 2>/dev/null \
    || date -u -v+2d +"%Y-%m-%dT18:00:00+00:00" 2>/dev/null \
    || echo "$(date -u +%Y-%m-%d)T18:00:00+00:00")

for i in "${!MOSQUE_IDS[@]}"; do
    mosque_id="${MOSQUE_IDS[$i]}"
    title="${EVENT_TITLES[$(( i % ${#EVENT_TITLES[@]} ))]} ${RUN_ID}"
    category="${EVENT_CATEGORIES[$(( i % ${#EVENT_CATEGORIES[@]} ))]}"
    description="${EVENT_DESCRIPTIONS[$(( i % ${#EVENT_DESCRIPTIONS[@]} ))]}"
    speaker="${SPEAKERS[$(( i % ${#SPEAKERS[@]} ))]}"

    speaker_field="null"
    if [ -n "$speaker" ]; then
        speaker_field="\"$speaker\""
    fi

    payload=$(cat <<JSON
{
    "create_event": {
        "title": "$title",
        "description": "$description",
        "category": "$category",
        "date": "$EVENT_DATE",
        "mosque": "$mosque_id",
        "speaker": $speaker_field,
        "recurrence_pattern": null,
        "recurrence_duration": null
    }
}
JSON
)

    api_post "/mosques/events/add-event" "$payload"

    if [ "$HTTP_CODE" != "200" ] && [ "$HTTP_CODE" != "201" ]; then
        echo "  FAILED: $mosque_id - HTTP $HTTP_CODE - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    if ! has_no_error "$RESPONSE_BODY"; then
        echo "  FAILED: $mosque_id - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    echo "  OK: $title -> $mosque_id"
done

echo ""

# ============================================================
# Step 3: favorite the mosques of the user
# ============================================================
echo "[3/4] Adding the mosques to the favorites of the user..."

api_get "/mosques/favorite"
FAVORITES_BODY="$RESPONSE_BODY"

if [ "$HTTP_CODE" != "200" ]; then
    echo "  FAILED: could not read the favorites - HTTP $HTTP_CODE - $FAVORITES_BODY"
    FAILURES=$((FAILURES + 1))
fi

for i in "${!MOSQUE_IDS[@]}"; do
    mosque_id="${MOSQUE_IDS[$i]}"

    # RELATE would create a second edge for a mosque that is already a
    # favorite, so the existing favorites are checked first.
    if printf '%s' "$FAVORITES_BODY" | grep -q "\"$mosque_id\""; then
        echo "  SKIP: $mosque_id is already a favorite"
        continue
    fi

    payload="{ \"mosque_id\": \"$mosque_id\" }"

    api_post "/mosques/add-favorite" "$payload"

    if [ "$HTTP_CODE" != "200" ] && [ "$HTTP_CODE" != "201" ]; then
        echo "  FAILED: $mosque_id - HTTP $HTTP_CODE - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    if ! has_no_error "$RESPONSE_BODY"; then
        echo "  FAILED: $mosque_id - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    echo "  OK: $mosque_id"
done

echo ""

# ============================================================
# Step 4: verify everything
# ============================================================
echo "[4/4] Verifying the mosques and their events..."

# Every mosque has to show up in the mosques around the user's coordinates.
# The endpoint only answers with the mosques that are within 5 km.
api_post "/mosques/fetch-mosques-for-location" \
    "{ \"lat\": $USER_LAT, \"lon\": $USER_LON, \"single_closest\": false }"

if [ "$HTTP_CODE" != "200" ]; then
    echo "  FAILED: could not fetch the nearby mosques - HTTP $HTTP_CODE - $RESPONSE_BODY"
    FAILURES=$((FAILURES + 1))
else
    for i in "${!MOSQUE_LABELS[@]}"; do
        name="${MOSQUE_LABELS[$i]}"
        if printf '%s' "$RESPONSE_BODY" | grep -qF "\"$name\""; then
            echo "  OK: $name is within ${RADIUS_KM} km of the user"
        else
            echo "  FAILED: $name was not found around the user's coordinates"
            FAILURES=$((FAILURES + 1))
        fi
    done
fi

# Every mosque has to host the event that was created for it.
for i in "${!MOSQUE_IDS[@]}"; do
    mosque_id="${MOSQUE_IDS[$i]}"
    title="${EVENT_TITLES[$(( i % ${#EVENT_TITLES[@]} ))]} ${RUN_ID}"
    name="${MOSQUE_LABELS[$i]}"

    api_post "/mosques/events/fetch-mosque-events" \
        "{ \"mosque_id\": \"$mosque_id\" }"

    if [ "$HTTP_CODE" != "200" ]; then
        echo "  FAILED: $mosque_id - HTTP $HTTP_CODE - $RESPONSE_BODY"
        FAILURES=$((FAILURES + 1))
        continue
    fi

    EVENT_OBJECTS=$(printf '%s' "$RESPONSE_BODY" | grep -o '{[^{}]*}')

    if printf '%s' "$EVENT_OBJECTS" | grep -F "\"$title\"" | grep -qF "\"$name\""; then
        echo "  OK: $name hosts \"$title\""
    else
        echo "  FAILED: $name does not host \"$title\""
        FAILURES=$((FAILURES + 1))
    fi
done

echo ""
echo "===================================================="
if [ "$FAILURES" -eq 0 ]; then
    echo "SUCCESS: ${#MOSQUE_IDS[@]} test mosques and their events were added"
else
    echo "FAILED: $FAILURES check(s) did not pass"
fi
echo "===================================================="

exit "$FAILURES"
