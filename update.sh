#!/usr/bin/env nix-shell
#! nix-shell -p cargo bash -i bash

# Update flake
nix flake update

# Update cargo
cargo update
