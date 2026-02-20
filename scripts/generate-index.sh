#!/usr/bin/env bash
# Regenerates providers/index.json from the current providers/*.json files.
# Run this whenever you add, remove, or bump the version of a recipe.
# The updated_at timestamp is only changed when the recipe list or versions change.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROVIDERS_DIR="$SCRIPT_DIR/../providers"
OUTPUT="$PROVIDERS_DIR/index.json"

python3 - "$PROVIDERS_DIR" "$OUTPUT" <<'PYEOF'
import json, os, sys, glob
from datetime import timezone, datetime

providers_dir = sys.argv[1]
output_path = sys.argv[2]

files = sorted(glob.glob(os.path.join(providers_dir, '*.json')))

recipes = []
for f in files:
    basename = os.path.basename(f)
    if basename == 'index.json':
        continue
    with open(f) as fh:
        data = json.load(fh)
    recipes.append({
        'id': data['id'],
        'file': basename,
        'version': data.get('version', '1.0.0'),
    })

# Read existing index to preserve updated_at when nothing changed
existing_recipes = []
existing_updated_at = None
if os.path.exists(output_path):
    with open(output_path) as fh:
        existing = json.load(fh)
    existing_recipes = existing.get('recipes', [])
    existing_updated_at = existing.get('updated_at')

if recipes == existing_recipes and existing_updated_at:
    updated_at = existing_updated_at
else:
    updated_at = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')

index = {
    'schema_version': '1',
    'updated_at': updated_at,
    'recipes': recipes,
}

with open(output_path, 'w') as fh:
    json.dump(index, fh, indent=2)
    fh.write('\n')

print(f"Generated {output_path} with {len(recipes)} recipes.")
PYEOF
