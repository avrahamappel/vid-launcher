#!/usr/bin/env bash

# Update flake
nix flake update

# Update cargo
cargo update

# Check that nix build succeeds
nix build --no-link
