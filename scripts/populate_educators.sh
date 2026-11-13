#!/bin/bash
set -euo pipefail

# ============================================================
# populate_educators.sh - Create educator records
# ============================================================
# Usage:
#   ./scripts/populate_educators.sh
#
# Ensures one educator account exists per name in the EDUCATORS list and
# writes the name -> record id mapping to scripts/data/educator_ids.json.
#
# Re-runs are safe: an educator that already exists (matched on
# display_name) is reused instead of being created a second time, so the
# ids in educator_ids.json stay stable and no duplicate accounts appear.
#
# Requires:
#   - SurrealDB running
#   - Credentials in .env (SURREAL_URL, SURREAL_USER, SURREAL_PASS,
#     SURREAL_NS, SURREAL_DB)
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DATA_DIR="$PROJECT_ROOT/scripts/data"
EDUCATORS_FILE="$DATA_DIR/educators.json"

SURREAL_USER=$(grep '^SURREAL_USER' "$PROJECT_ROOT/.env" | sed 's/.*= *//')
SURREAL_PASS=$(grep '^SURREAL_PASS' "$PROJECT_ROOT/.env" | sed 's/.*= *//')
SURREAL_URL=$(grep '^SURREAL_URL' "$PROJECT_ROOT/.env" | sed 's/.*= *//')
SURREAL_NS=$(grep '^SURREAL_NS' "$PROJECT_ROOT/.env" | sed 's/.*= *//')
SURREAL_DB=$(grep '^SURREAL_DB' "$PROJECT_ROOT/.env" | sed 's/.*= *//')

# Run one statement and print the raw JSON result.
sql() {
    curl -s -X POST "http://${SURREAL_URL}/sql" \
        -u "${SURREAL_USER}:${SURREAL_PASS}" \
        -H "surreal-ns: ${SURREAL_NS}" \
        -H "surreal-db: ${SURREAL_DB}" \
        -H "Accept: application/json" \
        -d "$1"
}

# Print the first record id of a query result ("table:id"), or nothing.
print_id() {
    python3 -c "
import json, sys
try:
    data = json.load(sys.stdin)
except Exception:
    sys.exit(0)
result = data[0].get('result') if data else None
if not result:
    sys.exit(0)
row = result[0]
if not isinstance(row, dict) or 'id' not in row:
    sys.exit(0)
rid = row['id']
if isinstance(rid, dict):
    tb = rid.get('tb', '')
    inner = rid.get('id')
    if isinstance(inner, dict):
        inner = inner.get('String', '')
    print('%s:%s' % (tb, inner))
else:
    print(rid)
"
}

echo "===================================================="
echo "Creating Educator Records"
echo "===================================================="
echo ""

EDUCATORS=(
    "SeekersGuidance"
    "Shaykh Faraz Rabbani"
    "Ustadh Abdullah Misra"
    "Shaykh Yahya Rhodus"
    "Shaykh Abdul-Rahim Reasat"
    "Ustadha Shireen Ahmed"
    "Quran.com"
    "Sunnah.com"
    "Imam al-Bukhari"
    "Imam Muslim"
    "Imam an-Nawawi"
    "Imam Malik"
    "Imam at-Tirmidhi"
    "Imam Ibn Majah"
    "Imam an-Nasa'i"
    "Imam Ahmad"
)

EDUCATOR_IDS_FILE="$DATA_DIR/educator_ids.json"
echo "[" > "$EDUCATOR_IDS_FILE"
FIRST=true

for name in "${EDUCATORS[@]}"; do
    # Reuse an existing account with the same display name (no duplicates).
    existing=$(sql "SELECT VALUE id FROM users WHERE display_name = '${name}' LIMIT 1;" | print_id)

    if [ -n "$existing" ]; then
        echo "  Exists: $name ($existing)"
        id="$existing"
    else
        echo "  Creating: $name"
        id=$(sql "CREATE users SET display_name = '${name}', password_hash = 'external', role = 'educator', created_at = time::now(), updated_at = time::now();" | print_id)
        echo "    ID: $id"
    fi

    if [ -n "$id" ]; then
        if [ "$FIRST" = true ]; then
            FIRST=false
        else
            echo "," >> "$EDUCATOR_IDS_FILE"
        fi
        printf '  {"name": "%s", "id": "%s"}' "$name" "$id" >> "$EDUCATOR_IDS_FILE"
    else
        echo "    ! could not resolve an id for ${name}" >&2
    fi
done

echo "" >> "$EDUCATOR_IDS_FILE"
echo "]" >> "$EDUCATOR_IDS_FILE"

echo ""
echo "===================================================="
echo "Educator creation complete!"
echo "===================================================="
echo ""
echo "Educator IDs saved to: $EDUCATOR_IDS_FILE"
cat "$EDUCATOR_IDS_FILE"
