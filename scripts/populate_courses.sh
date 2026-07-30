#!/bin/bash
set -euo pipefail

# ============================================================
# populate_courses.sh - Import Islamic educational courses
# ============================================================
# Usage:
#   ./scripts/populate_courses.sh
#
# Imports all course JSON files from scripts/data/courses/
# into the database using the import binary.
#
# Requires:
#   - SurrealDB running (docker-compose up -d)
#   - Features: --features ssr
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "===================================================="
echo "Importing Islamic Educational Courses"
echo "===================================================="
echo ""

# Check if SurrealDB is running
if ! curl -s http://127.0.0.1:8000/health > /dev/null 2>&1; then
    echo "WARNING: SurrealDB may not be running on port 8000"
    echo "Start it with: docker-compose up -d"
    echo ""
fi

# Count course files
COURSE_COUNT=$(ls -1 "$PROJECT_ROOT/scripts/data/courses/"*.json 2>/dev/null | wc -l)
echo "Found $COURSE_COUNT course files to import"
echo ""

# Run the import binary
echo "Running import..."
cd "$PROJECT_ROOT"
cargo run --bin import --features ssr -- --courses

echo ""
echo "===================================================="
echo "Course import complete!"
echo "===================================================="
