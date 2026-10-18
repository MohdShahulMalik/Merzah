#!/bin/bash

# Elevates the user recorded in scripts/.auth_state to the "app_admin" role
# directly in the database, so that authenticated requests through the server
# functions are treated as an app admin.
#
# The user is resolved, in order of preference, by:
#   1. TOKEN  -> sessions.session_token (SHA256 hex of the raw bearer token)
#   2. EMAIL  -> user_identifier (identifier_type = 'email')
#   3. MOBILE -> user_identifier (identifier_type = 'mobile')
#
# Requires SURREAL_USER, SURREAL_PASS, SURREAL_PORT, SURREAL_URL, SURREAL_DB and
# SURREAL_NS in .env (read at runtime, never sourced).

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
    value=$(printf '%s' "$value" | tr -d '\r')
    value=$(printf '%s' "$value" | sed -E "s/[[:space:]]+#.*$//")
    value=$(printf '%s' "$value" | sed -E "s/^\"(.*)\"$/\1/; s/^'(.*)'$/\1/")
    printf '%s' "$value"
}

# Quote a value for use inside a single-quoted SurrealQL string literal.
sql_str() {
    printf '%s' "$1" | sed "s/'/\\\\'/g"
}

SURREAL_URL=$(get_key "$ENV_FILE" SURREAL_URL)
SURREAL_PORT=$(get_key "$ENV_FILE" SURREAL_PORT)
SURREAL_USER=$(get_key "$ENV_FILE" SURREAL_USER)
SURREAL_PASS=$(get_key "$ENV_FILE" SURREAL_PASS)
SURREAL_DB=$(get_key "$ENV_FILE" SURREAL_DB)
SURREAL_NS=$(get_key "$ENV_FILE" SURREAL_NS)

TOKEN=$(get_key "$AUTH_STATE_FILE" TOKEN)
EMAIL=$(get_key "$AUTH_STATE_FILE" EMAIL)
MOBILE=$(get_key "$AUTH_STATE_FILE" MOBILE)

if [ -z "$SURREAL_USER" ] || [ -z "$SURREAL_PASS" ] || [ -z "$SURREAL_DB" ] || [ -z "$SURREAL_NS" ]; then
    echo "ERROR: SURREAL_USER, SURREAL_PASS, SURREAL_DB and SURREAL_NS must be set in .env"
    exit 1
fi

EMAIL_LC=$(printf '%s' "$EMAIL" | tr '[:upper:]' '[:lower:]' | tr -d '[:space:]')

# Strip everything except digits and a leading '+' (mirrors normalize_mobile).
MOBILE_NORM=$(printf '%s' "$MOBILE" | tr -d '[:space:]' | sed -E 's/[^0-9+]+//g; s/\+(.*)\+/+\1/g')

# SHA256 hex of the raw bearer token, matching hash_session_token().
TOKEN_HASH=""
if [ -n "$TOKEN" ]; then
    TOKEN_HASH=$(printf '%s' "$TOKEN" | sha256sum | awk '{print $1}')
fi

# Build the HTTP base from SURREAL_URL, turning the ws(s) scheme into http(s).
HTTP_BASE="${SURREAL_URL:-http://127.0.0.1:${SURREAL_PORT:-8000}}"
case "$HTTP_BASE" in
    ws://*)   HTTP_BASE="http://${HTTP_BASE#ws://}" ;;
    wss://*)  HTTP_BASE="https://${HTTP_BASE#wss://}" ;;
    http://*|https://*) ;;
    *)        HTTP_BASE="http://${HTTP_BASE}" ;;
esac
HTTP_BASE="${HTTP_BASE%/}"

# If SURREAL_URL carries no port, fall back to SURREAL_PORT.
host_part="${HTTP_BASE#*://}"
case "$host_part" in
    *:*) ;;
    *) [ -n "$SURREAL_PORT" ] && HTTP_BASE="${HTTP_BASE}:${SURREAL_PORT}" ;;
esac

AUTH_HEADER="Basic $(printf '%s:%s' "$SURREAL_USER" "$SURREAL_PASS" | base64 | tr -d '\n')"

run_sql() {
    curl -s -w "\n%{http_code}" -X POST "${HTTP_BASE}/sql" \
        -H "Accept: application/json" \
        -H "Content-Type: text/plain" \
        -H "surreal-ns: ${SURREAL_NS}" \
        -H "surreal-db: ${SURREAL_DB}" \
        -H "Authorization: ${AUTH_HEADER}" \
        --data-binary "$1"
}

echo "Resolving user from $AUTH_STATE_FILE on $HTTP_BASE (ns=$SURREAL_NS, db=$SURREAL_DB)"

USER_ID=""

try_lookup() {
    local label="$1"
    local query="$2"
    local response body http_code
    response=$(run_sql "$query")
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | sed '$d')
    if [ "$http_code" -ne 200 ]; then
        echo "  [$label] HTTP $http_code - $body"
        return 1
    fi
    local id
    id=$(printf '%s' "$body" | grep -oE 'users:[A-Za-z0-9_]+' | head -n1)
    if [ -n "$id" ]; then
        USER_ID="$id"
        echo "  [$label] matched $USER_ID"
        return 0
    fi
    echo "  [$label] no match"
    return 1
}

if [ -n "$TOKEN_HASH" ]; then
    try_lookup "token" \
        "SELECT VALUE user FROM sessions WHERE session_token = '${TOKEN_HASH}' LIMIT 1;" || true
fi

if [ -z "$USER_ID" ] && [ -n "$EMAIL_LC" ]; then
    try_lookup "email" \
        "SELECT VALUE user FROM user_identifier WHERE identifier_type = 'email' AND identifier_value = '$(sql_str "$EMAIL_LC")' LIMIT 1;" || true
fi

if [ -z "$USER_ID" ] && [ -n "$MOBILE_NORM" ]; then
    try_lookup "mobile" \
        "SELECT VALUE user FROM user_identifier WHERE identifier_type = 'mobile' AND identifier_value = '$(sql_str "$MOBILE_NORM")' LIMIT 1;" || true
fi

if [ -z "$USER_ID" ]; then
    echo "ERROR: could not resolve a user from TOKEN/EMAIL/MOBILE in $AUTH_STATE_FILE"
    exit 1
fi

echo "Elevating $USER_ID to app_admin..."

update_response=$(run_sql "UPDATE ${USER_ID} SET role = 'app_admin', updated_at = time::now();")
update_code=$(echo "$update_response" | tail -n1)
update_body=$(echo "$update_response" | sed '$d')
if [ "$update_code" -ne 200 ] || printf '%s' "$update_body" | grep -q '"status":"ERR"'; then
    echo "FAILED to update $USER_ID: HTTP $update_code - $update_body"
    exit 1
fi

verify_response=$(run_sql "SELECT id, role FROM ${USER_ID};")
verify_body=$(echo "$verify_response" | sed '$d')
echo "User record: $verify_body"

if printf '%s' "$verify_body" | grep -q '"app_admin"'; then
    echo "SUCCESS: $USER_ID is now an app_admin"
else
    echo "WARNING: could not confirm app_admin role from response above"
    exit 1
fi
