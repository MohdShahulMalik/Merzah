#!/bin/bash
set -euo pipefail

# ============================================================
# populate_salafi_educators.sh - Create organisation educator accounts
# ============================================================
# Usage:
#   ./scripts/populate_salafi_educators.sh
#
# Creates the organisation educator accounts referenced by the course
# seed files in scripts/data/courses/ (see scripts/data/educator_ids.json).
# Mostly Salafi/Athari organisations, plus halal-finance media orgs and
# Islamic tool publishers used by the halal/worldly courses.
#
# Record ids are FIXED slugs (e.g. users:garden_of_ilm), so this script is
# idempotent: re-running it reuses the existing accounts instead of
# creating duplicates, and the ids in the seed files keep resolving.
#
# These are attribution-only accounts: role "educator", unusable password
# hash, no login identifiers - they can never sign in.
#
# Unlike populate_educators.sh, this script does NOT touch the existing
# random-id educators; it only manages its own rows.
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

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

echo "===================================================="
echo "Creating Salafi/Athari Educator Records"
echo "===================================================="
echo ""

# name|record_id - keep in sync with the entries in scripts/data/educator_ids.json
EDUCATORS=(
    "Garden of Ilm|users:garden_of_ilm"
    "Newcastle Central Mosque|users:newcastle_central_mosque"
    "AMAU Academy|users:amau_academy"
    "Ulul Absaar Project|users:ulul_absaar_project"
    "Muwahhideen Publications|users:muwahhideen_publications"
    "Salafi Matloob|users:salafi_matloob"
    "Green Lane Masjid|users:green_lane_masjid"
    "Forgotten Light|users:forgotten_light"
    "Salafi Publications|users:salafi_publications"
    "The Patient One|users:the_patient_one"
    "Umm Uthmaan al-Majariyyah|users:umm_uthmaan_al_majariyyah"
    "Assim Al-Hakeem|users:assim_al_hakeem"
    "TROID|users:troid"
    "E-Tilawah Academy|users:etilawah_academy"
    "Islamic Home Schooling Academy - New Zealand|users:ihsan_academy"
    "Madeenah Academy|users:madeenah_academy"
    "Islamic Finance Guru|users:ifg"
    "Miftaah Institute|users:miftaah_institute"
)

created=0
existing=0

for entry in "${EDUCATORS[@]}"; do
    name="${entry%%|*}"
    id="${entry##*|}"

    # Already there? Then leave the account alone - no duplicates.
    found=$(sql "SELECT VALUE id FROM users WHERE id = ${id} LIMIT 1;" \
        | python3 -c "
import json, sys
try:
    data = json.load(sys.stdin)
except Exception:
    sys.exit(0)
result = data[0].get('result') if data else None
if result:
    print(result[0])
")
    if [ -n "$found" ]; then
        echo "  Exists: $name ($id)"
        existing=$((existing + 1))
        continue
    fi

    echo "  Creating: $name ($id)"
    sql "UPSERT ${id} CONTENT { display_name: '${name}', password_hash: 'UNUSABLE_SEED_ACCOUNT', role: 'educator', created_at: time::now(), updated_at: time::now() };" > /dev/null
    created=$((created + 1))
done

echo ""
echo "===================================================="
echo "Salafi/Athari educator creation complete!"
echo "  created: ${created} | already present: ${existing}"
echo "===================================================="
echo ""
echo "These fixed ids must match the entries in scripts/data/educator_ids.json"
echo "so the course seed files resolve their educator references."
