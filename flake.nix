{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { flake-utils
    , nixpkgs
    , fenix
    , naersk
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk {};

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          lockFile = ./Cargo.lock;
        };
      in
      {
        defaultPackage = (pkgs.callPackage ./. {
          naersk = naersk';
        });

        devShell = pkgs.mkShell {
          inherit cargoDeps;
          nativeBuildInputs = with pkgs; [
            rustPlatform.cargoSetupHook
            rust-analyzer
            (pkgs.fenix.stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            glib
            gtk4
            pkg-config
            cargo-bump
          ];
        };
      }
    );
}
