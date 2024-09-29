#!/usr/bin/env bash

set -o pipefail

# Update flake
nix flake update

# Update cargo
cargo update

# Update cargo hash if necessary
if ! output="$(nix build --no-link |& tee /dev/tty)"; then
    # Find updated hash
    new_hash="$(echo "$output" | awk '/got:/ {print $2}')"
    echo "Updating Cargo dependencies hash to [$new_hash]"

    sed -i "s/cargoHash = \".*\";/cargoHash = \"$new_hash\";/" default.nix
fi
