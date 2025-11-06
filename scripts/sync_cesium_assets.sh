#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEFAULT_VERSION="1.135"
VERSION="${1:-$DEFAULT_VERSION}"

VENDOR_BASE="$ROOT_DIR/vendor/Cesium/$VERSION"
VENDOR_BUILD="$VENDOR_BASE/Build/Cesium"
LEGACY_BUILD="$ROOT_DIR/Cesium-$VERSION/Build/Cesium"
ZIP_PATH="$ROOT_DIR/Cesium-$VERSION.zip"

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

ensure_vendor_copy() {
  if [[ -d "$VENDOR_BUILD" ]]; then
    echo "Using vendorized Cesium build at $VENDOR_BUILD"
    return
  fi

  if [[ -d "$LEGACY_BUILD" ]]; then
    echo "Vendorizing existing Cesium build from $LEGACY_BUILD"
    mkdir -p "$VENDOR_BASE"
    cp -R "$ROOT_DIR/Cesium-$VERSION"/Build "$VENDOR_BASE/"
    return
  fi

  echo "ERROR: Cesium build not found for version $VERSION." >&2
  echo "Download the official Cesium package (e.g. $ZIP_PATH) and extract it" >&2
  echo "so that either $VENDOR_BUILD or $LEGACY_BUILD exists before rerunning." >&2
  exit 1
}

main() {
  ensure_vendor_copy

  local source_dir="$VENDOR_BUILD"
  local examples_dir="$ROOT_DIR/examples"

  if [[ ! -d "$examples_dir" ]]; then
    echo "No examples directory found at $examples_dir; nothing to sync."
    exit 0
  fi

  local synced_any=false

  while IFS= read -r -d '' example; do
    local target="$example/public/Cesium"
    echo "Syncing Cesium assets to $target"
    link_or_copy_build "$source_dir" "$target"
    synced_any=true
  done < <(find "$examples_dir" -mindepth 1 -maxdepth 1 -type d -print0)

  if [[ "$synced_any" == false ]]; then
    echo "No example directories found under $examples_dir."
  fi
}

main "$@"
