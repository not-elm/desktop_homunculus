#!/usr/bin/env bash
# Usage: compute-tag.sh <version>
# Outputs the npm dist-tag for a given semver version.
# Pre-release versions (alpha, beta, rc) get their label as tag; others get "latest".
VERSION="$1"
if echo "$VERSION" | grep -qE -- '-(alpha|beta|rc)'; then
  echo "$VERSION" | sed -E 's/.*-(alpha|beta|rc).*/\1/'
else
  echo "latest"
fi
