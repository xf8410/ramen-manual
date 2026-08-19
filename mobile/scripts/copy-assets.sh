#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DEST="$ROOT/mobile/app/src/main/assets"
mkdir -p "$DEST/gamedata"

for file in cardDB.json constants.json default_config.toml events.json scenario_ramen.json text_data_dict.json umaDB.json; do
  cp "$ROOT/$file" "$DEST/gamedata/$file"
done
cp "$ROOT/default_config.toml" "$DEST/gamedata/default_config.toml"
# The upstream loader expects game_config.toml at the working-directory root.
if [[ -f "$ROOT/game_config.toml" ]]; then
  cp "$ROOT/game_config.toml" "$DEST/game_config.toml"
else
  cp "$ROOT/default_config.toml" "$DEST/game_config.toml"
fi
