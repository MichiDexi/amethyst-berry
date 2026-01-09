#!/usr/bin/env bash
set -e

TERM="kitty"
BASE="$HOME/workspace/abt/"

spawn() {
  local ws="$1"
  local dir="$2"
  local cmd="$3"

	sleep 0.3
  hyprctl dispatch workspace "$ws"
  sleep 0.3

  if [[ -n "$cmd" ]]; then
    kitty --hold bash -lc "\"cd \\\"$dir\\\" && $cmd\"" &
  else
    $TERM --directory "$dir" &
  fi
}


# Workspace 1 – src/
spawn 1 "$BASE/src/"
spawn 1 "$BASE/src/"

# Workspace 2 – Parser
spawn 2 "$BASE/src/abt/"
spawn 2 "$BASE/src/helpers/"

# Workspace 3 – Menus
spawn 3 "$BASE/src/menus/"
spawn 3 "$BASE/src/helpers/"

# Workspace 4 – Documentation and other stuff
spawn 4 "$BASE/"
spawn 4 "$BASE/docs/"
