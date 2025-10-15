#!/bin/bash

set -eo pipefail

EXEC_RELEASE=false
if [[ "${2:-}" == "--execute" ]]; then
  EXEC_RELEASE=true
fi
VERSION="$1"

if [[ -z "$VERSION" ]]; then
  echo "Missing new version"
  exit 1
fi

if [[ "$EXEC_RELEASE" = true ]]; then
  echo "Starting pre-release process for version ${VERSION}..."

  echo "Generating changelog..."
  git cliff --tag "$VERSION" -o "CHANGELOG.md"

  git add "CHANGELOG.md"

  echo "Committing the new changelog"
  git commit -m "updated changelog"
fi
