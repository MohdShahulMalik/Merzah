#!/bin/bash
set -euo pipefail

# ============================================================
# populate_educators.sh - Create educator records for courses
# ============================================================
# Usage:
#   ./scripts/populate_educators.sh
#
# Creates educator user records in the database for external
# course providers. Outputs the educator IDs to a JSON file
# that can be used to update course JSON files.
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DATA_DIR="$PROJECT_ROOT/scripts/data"
EDUCATORS_FILE="$DATA_DIR/educators.json"

echo "===================================================="
echo "Creating Educator Records"
echo "===================================================="
echo ""

# Check if SurrealDB is running
if ! curl -s http://127.0.0.1:8000/health > /dev/null 2>&1; then
    echo "ERROR: SurrealDB is not running on port 8000"
    echo "Start it with: docker-compose up -d"
    exit 1
fi

# Educators to create
declare -A EDUCATORS
EDUCATORS["seekersguidance"]="SeekersGuidance"
EDUCATORS["shaykh-faraz-rabbani"]="Shaykh Faraz Rabbani"
EDUCATORS["ustadh-abdullah-misra"]="Ustadh Abdullah Misra"
EDUCATORS["shaykh-yahya-rhodus"]="Shaykh Yahya Rhodus"
EDUCATORS["shaykh-abdul-rahim-reasat"]="Shaykh Abdul-Rahim Reasat"
EDUCATORS["ustadha-shireen-ahmed"]="Ustadha Shireen Ahmed"
EDUCATORS["quran-com"]="Quran.com"
EDUCATORS["sunnah-com"]="Sunnah.com"
EDUCATORS["imam-al-bukhari"]="Imam al-Bukhari"
EDUCATORS["imam-muslim"]="Imam Muslim"
EDUCATORS["imam-an-nawawi"]="Imam an-Nawawi"
EDUCATORS["imam Malik"]="Imam Malik"
EDUCATORS["imam-at-tirmidhi"]="Imam at-Tirmidhi"
EDUCATORS["imam-ibn-majah"]="Imam Ibn Majah"
EDUCATORS["imam-an-nasai"]="Imam an-Nasa'i"
EDUCATORS["imam-ahmad"]="Imam Ahmad"

echo "Creating educators in database..."
echo ""

# Create educators and collect IDs
for key in "${!EDUCATORS[@]}"; do
    name="${EDUCATORS[$key]}"
    echo "  Creating: $name"
    
    response=$(curl -s -X POST "http://127.0.0.1:8000/sql" \
        -H "Content-Type: application/json" \
        -H "Accept: application/json" \
        -d "CREATE users SET display_name = '$name', password_hash = 'external', role = 'educator', created_at = time::now(), updated_at = time::now();" 2>/dev/null || echo '{"error":"failed"}')
    
    echo "    Response: $response"
done

echo ""
echo "===================================================="
echo "Educator creation complete!"
echo "===================================================="
echo ""
echo "To get educator IDs, run:"
echo "  surreal sql --endpoint ws://127.0.0.1:5100 --username root --password root \\"
echo "    --database merzah --namespace merzah --query 'SELECT id, display_name FROM users WHERE role = \"educator\"'"
