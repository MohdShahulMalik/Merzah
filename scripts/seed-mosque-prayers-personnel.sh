#!/usr/bin/env bash
# Seed all mosques within a 5 km radius of a test location with:
#   1. adhan + jamat prayer times
#   2. mosque personnel (an imam and a muazzin user account each)
#
# Usage:
#   USER_LAT=... USER_LON=... scripts/seed-mosque-prayers-personnel.sh
#   ...or just: scripts/seed-mosque-prayers-personnel.sh (reads USER_LAT/USER_LON
#   from the app's .env in the repo root when not exported in your shell)
#
# Env:
#   USER_LAT / USER_LON  (required, via env or the app .env)  center of the 5 km search radius
#   TOKEN                 optional   session token; sourced from .auth_state if unset
#   BASE                  optional   default http://localhost:3000
#
# The session token must belong to an app admin (personnel/prayer updates
# require mosque-admin privileges, which app admins always have).

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

env_value() { # env_value <key> — echo the numeric value of KEY from the app .env (never printed)
  python3 -c "
import re, sys
try:
    rx = re.compile(r'^\s*' + re.escape(sys.argv[2]) + r'\s*=\s*[\x27\"]?([-0-9.]+)[\x27\"]?\s*(?:#.*)?\$')
    for line in open(sys.argv[1], encoding='utf-8'):
        m = rx.match(line)
        if m:
            print(m.group(1))
            sys.exit()
except FileNotFoundError:
    pass
" "$SCRIPT_DIR/../.env" "$1"
}

USER_LAT=${USER_LAT:-}
USER_LON=${USER_LON:-}
if [ -z "$USER_LAT" ] || [ -z "$USER_LON" ]; then
  if [ -f "$SCRIPT_DIR/../.env" ]; then
    USER_LAT="${USER_LAT:-$(env_value USER_LAT)}"
    USER_LON="${USER_LON:-$(env_value USER_LON)}"
    # some .env variants carry the coordinates with an _F suffix
    USER_LAT="${USER_LAT:-$(env_value USER_LAT_F)}"
    USER_LON="${USER_LON:-$(env_value USER_LON_F)}"
  fi
fi
USER_LAT=${USER_LAT:?'USER_LAT not set — export it inline, or drop it in the app .env'}
USER_LON=${USER_LON:?'USER_LON not set — export it inline, or drop it in the app .env'}
BASE=${BASE:-http://localhost:3000}

if [ -z "${TOKEN:-}" ]; then
  # Pull the session token in a subshell so its file is never displayed.
  TOKEN="$( (set -a; . "$SCRIPT_DIR/.auth_state"; echo "$TOKEN") )"
fi
if [ -z "$TOKEN" ]; then
  echo "ERROR: no session TOKEN available" >&2
  exit 1
fi

PERSONNEL_PASSWORD="MerzahTest2024"

post() { # post <path> <json-body>
  curl -s -X POST "$BASE$1" \
    -H "Cookie: __Host-session=$TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --data "$2"
}
post_authorized() { # same session, but with an arbitrary caller cookie (personnel users)
  local cookie_token="$1"; shift
  local path="$1"; shift
  curl -s -X POST "$BASE$path" \
    -H "Cookie: __Host-session=$cookie_token" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --data "$1"
}
refresh_token() { # refresh the admin session via the login helper
  ./scripts/login.sh >/dev/null 2>&1
  TOKEN="$( (set -a; . "$SCRIPT_DIR/.auth_state"; echo "$TOKEN") )"
}
patch() { # patch <path> <json-body>
  curl -s -X PATCH "$BASE$1" \
    -H "Cookie: __Host-session=$TOKEN" \
    -H "Content-Type: application/json" \
    -H "Accept: application/json" \
    --data "$2"
}
_field() { # _field <json> <python-expr using d=>
  python3 -c "
import json, sys
try:
    d = json.loads(sys.argv[1])
except Exception:
    print('')
    sys.exit()
expr = sys.argv[2]
try:
    out = eval(expr)
    # treat missing keys (None) and empty strings identically
    if out is None or out == '':
        print('')
    elif isinstance(out, float):
        print(repr(out))
    else:
        print(out)
except Exception:
    print('')
" "$1" "$2"
}

slug_of() { # deterministic short tag from a mosque id, stable across re-runs
  echo "${1#mosques:}" | cut -c1-12
}

say() { echo "[$(date +%H:%M:%S)] $*"; }

echo "Fetching mosques within 5 km of ($USER_LAT, $USER_LON)..."
MOSQUES_JSON="$(post /mosques/fetch-mosques-for-location \
  "{\"lat\":$USER_LAT,\"lon\":$USER_LON,\"single_closest\":false}")"

MOSQUE_COUNT=$(_field "$MOSQUES_JSON" "len(d['data']['MosquesVec'])")
if [ -z "$MOSQUE_COUNT" ] || [ "$MOSQUE_COUNT" = "0" ]; then
  echo "ERROR: no mosques found near ($USER_LAT, $USER_LON). Response:" >&2
  echo "$MOSQUES_JSON" | head -c 300 >&2
  exit 1
fi
say "Found $MOSQUE_COUNT mosque(s)"

# Flatten the mosque list into "id|name" lines.
MOSQUE_LIST="$(python3 -c "
import json, sys
d = json.loads(sys.argv[1])
for m in d['data']['MosquesVec']:
    print(m['id'] + '|' + (m.get('name') or 'unnamed'))
" "$MOSQUES_JSON")"

index=0
overall_ok=0
overall_fail=0

while IFS='|' read -r MOSQUE_ID MOSQUE_NAME; do
  index=$((index + 1))
  echo
  say "($index/$MOSQUE_COUNT) $MOSQUE_NAME — $MOSQUE_ID"

  minute=$(( (index * 3) % 55 ))   # deterministic stagger per mosque

  printf -v adhan '%s' "{
    \"fajr\": \"05:$(printf '%02d' $minute):00\",
    \"dhuhr\": \"12:$(printf '%02d' $(( (minute + 10) % 60 ))):00\",
    \"asr\": \"15:$(printf '%02d' $(( (minute + 20) % 60 ))):00\",
    \"maghrib\": \"18:$(printf '%02d' $(( (minute + 25) % 60 ))):00\",
    \"isha\": \"19:$(printf '%02d' $(( (minute + 35) % 60 ))):00\",
    \"jumuah\": \"12:$(printf '%02d' $(( (minute + 40) % 60 ))):00\"
  }"
  printf -v jamat '%s' "{
    \"fajr\": \"06:$(printf '%02d' $(( (minute + 15) % 60 ))):00\",
    \"dhuhr\": \"13:$(printf '%02d' $(( (minute + 5) % 60 ))):00\",
    \"asr\": \"16:$(printf '%02d' $(( (minute + 15) % 60 ))):00\",
    \"maghrib\": \"19:$(printf '%02d' $(( (minute + 5) % 60 ))):00\",
    \"isha\": \"20:$(printf '%02d' $(( (minute + 10) % 60 ))):00\",
    \"jumuah\": \"13:$(printf '%02d' $(( (minute + 20) % 60 ))):00\"
  }"

  # ---------- 1. prayer times ----------
  TIMES_RESP="$(patch /mosques/update-adhan-jamat-times \
    "{\"mosque_id\":\"$MOSQUE_ID\",
      \"prayer_times\":{\"adhan_times\":$adhan,\"jamat_times\":$jamat}}")"
  TIMES_ERR=$(_field "$TIMES_RESP" "d.get('error')")
  if [ "$TIMES_RESP" = "" ] || [ "$TIMES_ERR" != "" ]; then
    # stale admin session: refresh once and retry
    if echo "$TIMES_ERR" | grep -q "session"; then
      say "admin session stale; refreshing"
      refresh_token
      TIMES_RESP="$(patch /mosques/update-adhan-jamat-times \
        "{\"mosque_id\":\"$MOSQUE_ID\",
          \"prayer_times\":{\"adhan_times\":$adhan,\"jamat_times\":$jamat}}")"
      TIMES_ERR=$(_field "$TIMES_RESP" "d.get('error')")
    fi
  fi
  if [ "$TIMES_RESP" = "" ] || [ "$TIMES_ERR" != "" ]; then
    echo "  [WARN] times update failed: $TIMES_ERR"
  else
    echo "  [OK] prayer times set"
  fi

  # ---------- 2. personnel ----------
  slug="$(slug_of "$MOSQUE_ID")"
  for role in imam muazzin; do
    EMAIL="${role}.${slug}@merzah-test.example"
    PERSON_NAME="$MOSQUE_NAME $role"

    # try to register; fall back to login when the account already exists
    REG_RESP="$(post /auth/register \
      "{\"form\":{\"name\":\"$PERSON_NAME\",
        \"identifier\":{\"identifier_type\":\"email\",\"identifier_value\":\"$EMAIL\"},
        \"password\":\"$PERSONNEL_PASSWORD\",
        \"platform\":\"mobile\"}}")"
    REG_ERR=$(_field "$REG_RESP" "d.get('error')")
    PERSON_TOKEN=$(_field "$REG_RESP" "d.get('data')")

    if [ -z "$REG_ERR" ] && [ -n "$PERSON_TOKEN" ]; then
      echo "  [OK] registered $role: $EMAIL"
    else
      LOGIN_RESP="$(post /auth/login \
        "{\"form\":{\"identifier\":{\"identifier_type\":\"email\",\"identifier_value\":\"$EMAIL\"},
          \"password\":\"$PERSONNEL_PASSWORD\",
          \"platform\":\"mobile\"}}")"
      PERSON_TOKEN=$(_field "$LOGIN_RESP" "d.get('data')")
      if [ -n "$PERSON_TOKEN" ]; then
        echo "  [SKIP] $role account exists, logged in: $EMAIL"
      else
        echo "  [WARN] could not create/login $role ($EMAIL): ${REG_ERR}"
        continue
      fi
    fi

    ME_JSON="$(post_authorized "$PERSON_TOKEN" /auth/me '{}')"
    PERSON_ID=$(_field "$ME_JSON" "d.get('data', {}).get('id') if isinstance(d.get('data'), dict) else ''")

    if [ -z "$PERSON_ID" ]; then
      echo "  [WARN] could not resolve $role user id"
      continue
    fi

    ASSIGN_RESP="$(patch /mosques/update-personnel \
      "{\"person_type\":\"$role\",
        \"person_id\":\"$PERSON_ID\",
        \"mosque_id\":\"$MOSQUE_ID\"}")"
    ASSIGN_ERR=$(_field "$ASSIGN_RESP" "d.get('error')")
    if [ -z "$ASSIGN_ERR" ]; then
      echo "  [OK] $role assigned: $PERSON_ID ← $EMAIL"
    else
      echo "  [WARN] $role assignment failed: $ASSIGN_ERR"
    fi
  done
done <<< "$MOSQUE_LIST"

echo
say "Done. $MOSQUE_COUNT mosque(s) processed."
