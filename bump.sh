#!/bin/bash

set -e

if ! command -v git-cliff &>/dev/null; then
  echo "Error: git-cliff is not installed."
  exit 1
fi

if ! command -v gh &>/dev/null; then
  echo "Error: GitHub CLI (gh) is not installed."
  exit 1
fi

if ! gh auth status &>/dev/null; then
  echo "Error: GitHub CLI is not authenticated."
  exit 1
fi

if ! command -v cargo &>/dev/null; then
  echo "Error: Cargo is not installed."
  exit 1
fi

if [ -z "$1" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

VERSION="$1"
TAG="v$VERSION"

if sed --version >/dev/null 2>&1; then
  sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
else
  sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
fi

cargo update -p catalyst --precise "$VERSION"

git add Cargo.toml Cargo.lock
git commit -m "Bump version to $VERSION"
git push origin main

if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Tag $TAG already exists."
  exit 1
fi

git tag "$TAG"
git push origin "$TAG"

NOTES=$(git-cliff --tag "$TAG")

gh release create "$TAG" --title "$TAG" --notes "$NOTES"

cargo doc --no-deps --document-private-items --all-features
cargo publish
