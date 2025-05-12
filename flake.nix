{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { flake-utils
    , nixpkgs
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        defaultPackage = pkgs.callPackage ./. { };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-analyzer
            glib
            gtk4
            pkg-config
            cargo-bump
            cargo
            clippy
            rustc
            rustfmt
            bacon
          ];
        };
      }
    );
}
