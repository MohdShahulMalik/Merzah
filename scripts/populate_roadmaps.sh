#!/bin/bash
set -euo pipefail

# ============================================================
# populate_roadmaps.sh - Import roadmaps and frameworks
# ============================================================
# Usage:
#   ./scripts/populate_roadmaps.sh
#
# Imports all roadmap JSON files from scripts/data/roadmaps/
# and framework JSON files from scripts/data/frameworks/
# into the database using the import binary.
#
# Requires:
#   - SurrealDB running (docker-compose up -d)
#   - Courses already imported (run populate_courses.sh first)
#   - Features: --features ssr
# ============================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "===================================================="
echo "Importing Roadmaps and Frameworks"
echo "===================================================="
echo ""

# Check if SurrealDB is running
if ! curl -s http://127.0.0.1:8000/health > /dev/null 2>&1; then
    echo "WARNING: SurrealDB may not be running on port 8000"
    echo "Start it with: docker-compose up -d"
    echo ""
fi

# Count files
ROADMAP_COUNT=$(ls -1 "$PROJECT_ROOT/scripts/data/roadmaps/"*.json 2>/dev/null | wc -l)
FRAMEWORK_COUNT=$(ls -1 "$PROJECT_ROOT/scripts/data/frameworks/"*.json 2>/dev/null | wc -l)
echo "Found $ROADMAP_COUNT roadmap files and $FRAMEWORK_COUNT framework files"
echo ""

# Run the import binary
echo "Running import..."
cd "$PROJECT_ROOT"
cargo run --bin import --features ssr -- --roadmaps --frameworks

echo ""
echo "===================================================="
echo "Roadmap and framework import complete!"
echo "===================================================="
