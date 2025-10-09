#!/bin/bash

set -eo pipefail

EXEC_RELEASE=false
if [[ "${2:-}" == "--exec" ]]; then
  EXEC_RELEASE=true
  echo "Execution flag '--exec' detected."
fi
VERSION="$1"

if [[ -z "$VERSION" ]]; then
  echo "Missing new version"
  exit 1
fi

if [[ -n $(git status --porcelain) ]]; then
  echo "Error: Your working directory is not clean. Please commit or stash your changes."
  exit 1
fi

echo "Running tests..."

cargo test --all-features -- -q --nocapture

if [[ "$EXEC_RELEASE" = true ]]; then
  echo "Starting pre-release process for version ${VERSION}..."

  echo "Generating changelog..."
  git cliff --tag "$VERSION" -o "CHANGELOG.md"

  echo "Generating changelog for proto-types..."
  (cd proto_types && git cliff --tag "$VERSION" -o "CHANGELOG.md")

  CHANGES_WERE_MADE=false

  if [[ -n $(git status --porcelain "CHANGELOG.md") ]]; then
    git add "CHANGELOG.md"
    CHANGES_WERE_MADE=true
  fi

  if [[ -n $(git status --porcelain "proto_types/CHANGELOG.md") ]]; then
    git add "proto_types/CHANGELOG.md"
    CHANGES_WERE_MADE=true
  fi

  if [[ "$CHANGES_WERE_MADE" = true ]]; then
    echo "Committing the new changelog"
    git commit -m "update changelog"
  fi

  cargo release "$VERSION" --execute
else
  cargo release "$VERSION"
fi

echo "Release routine finished!"
