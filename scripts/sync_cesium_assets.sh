#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEFAULT_VERSION="1.135"
VERSION="${1:-$DEFAULT_VERSION}"

VENDOR_BASE="$ROOT_DIR/vendor/Cesium/$VERSION"
VENDOR_BUILD="$VENDOR_BASE/Build/Cesium"
CESIUM_DTS="$VENDOR_BASE/Source/Cesium.d.ts"

link_or_copy_build() {
  local source_dir="$1"
  local target_dir="$2"

  mkdir -p "$(dirname "$target_dir")"

  if ln -sfn "$source_dir" "$target_dir" 2>/dev/null; then
    return
  fi

  echo "Symlink unavailable; copying Cesium build into $target_dir"
  rm -rf "$target_dir"
  mkdir -p "$target_dir"

  if command -v rsync >/dev/null 2>&1; then
    rsync -a --delete "$source_dir/" "$target_dir/"
  else
    cp -R "$source_dir/." "$target_dir/"
  fi
}

validate_cesium_build() {
  local build_dir="$1"

  if [[ ! -f "$build_dir/Cesium.js" ]]; then
    echo "ERROR: Invalid Cesium build - missing Cesium.js" >&2
    return 1
  fi

  if [[ ! -d "$build_dir/Workers" ]]; then
    echo "ERROR: Invalid Cesium build - missing Workers directory" >&2
    return 1
  fi

  if [[ ! -d "$build_dir/Assets" ]]; then
    echo "ERROR: Invalid Cesium build - missing Assets directory" >&2
    return 1
  fi

  return 0
}

ensure_vendor_copy() {
  if [[ ! -d "$VENDOR_BUILD" ]]; then
    echo "ERROR: Cesium build not found at $VENDOR_BUILD" >&2
    echo "" >&2
    echo "To install Cesium:" >&2
    echo "  1. Download Cesium-$VERSION.zip from https://cesium.com/downloads/" >&2
    echo "  2. Extract to $ROOT_DIR/" >&2
    echo "  3. Move Cesium-$VERSION/Build to $VENDOR_BASE/" >&2
    echo "" >&2
    echo "Expected structure: $VENDOR_BUILD/Cesium.js" >&2
    exit 1
  fi

  if ! validate_cesium_build "$VENDOR_BUILD"; then
    echo "ERROR: Cesium build at $VENDOR_BUILD is incomplete or corrupted" >&2
    exit 1
  fi

  echo "Using Cesium build at $VENDOR_BUILD"
}

copy_typescript_definitions() {
  if [[ -f "$CESIUM_DTS" ]]; then
    local target="$ROOT_DIR/Cesium.d.ts"
    echo "Copying TypeScript definitions to $target"
    cp "$CESIUM_DTS" "$target"
  else
    echo "Warning: Cesium.d.ts not found at $CESIUM_DTS" >&2
  fi
}

main() {
  ensure_vendor_copy
  copy_typescript_definitions

  local source_dir="$VENDOR_BUILD"
  local examples_dir="$ROOT_DIR/examples"

  if [[ ! -d "$examples_dir" ]]; then
    echo "No examples directory found at $examples_dir; nothing to sync."
    exit 0
  fi

  local synced_any=false

  while IFS= read -r -d '' example; do
    local target="$example/public/Cesium"
    local example_name="$(basename "$example")"

    # cargo-leptos doesn't handle directory symlinks, so force copy for server examples
    if [[ "$example_name" == "with-server" ]]; then
      echo "Syncing Cesium assets to $target (forcing copy for cargo-leptos compatibility)"
      # Remove symlink if it exists
      rm -rf "$target"
      mkdir -p "$target"
      if command -v rsync >/dev/null 2>&1; then
        rsync -a --delete "$source_dir/" "$target/"
      else
        cp -R "$source_dir/." "$target/"
      fi
    else
      echo "Syncing Cesium assets to $target"
      link_or_copy_build "$source_dir" "$target"
    fi
    synced_any=true
  done < <(find "$examples_dir" -mindepth 1 -maxdepth 1 -type d -print0)

  if [[ "$synced_any" == false ]]; then
    echo "No example directories found under $examples_dir."
  fi
}

main "$@"
