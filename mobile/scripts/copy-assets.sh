#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DEST="$ROOT/mobile/app/src/main/assets"
mkdir -p "$DEST/gamedata"

for file in cardDB.json constants.json default_config.toml events.json scenario_ramen.json text_data_dict.json umaDB.json; do
  cp "$ROOT/$file" "$DEST/gamedata/$file"
done

# The repository root default_config.toml is an Onsen configuration. Never copy it
# as game_config.toml: the mobile core requires scenario=ramen.
cp "$ROOT/mobile/assets/game_config.toml" "$DEST/game_config.toml"
