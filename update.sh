#!/usr/bin/env bash

# Update flake
nix flake update

# Update cargo
cargo update

# Update patch version if dependencies changed
if [[ -n "$(git diff --name-only)" ]]; then
  cargo bump patch
  cargo check # So the change will propagate to Cargo.lock
fi
