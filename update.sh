#!/usr/bin/env bash

# Update flake
nix flake update

# Update cargo
cargo update

# Update patch version if Cargo dependencies changed
if [[ "$(git diff --name-only)" =~ Cargo.lock ]]; then
  cargo bump patch
  # The change will propagate to Cargo.lock when we run cargo test
fi

cargo test
