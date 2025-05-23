#!/usr/bin/env bash
set -e
TAG=${1:-"v0.16.0"}               # default tag; override with e.g. v0.17.1
(
  cd docs/bevy-src
  git fetch --tags
  git checkout "$TAG"
)
cargo clean -p bevy
cargo doc --package bevy --no-deps --workspace --release
rm -rf docs/bevy-html
cp -r target/doc docs/bevy-html
echo "bevy ${TAG} | rustc $(rustc --version --short) | $(date +%F)" \
  > docs/VERSION.txt
echo "Docs updated to Bevy ${TAG}" 