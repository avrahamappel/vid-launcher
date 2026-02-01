{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self
    , flake-utils
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
      rec {
        defaultPackage = pkgs.callPackage ./. {
          version = if self ? shortRev then self.shortRev else "dirty";
        };

        devShell = pkgs.mkShell {
          packages = with pkgs; [
            bacon
            cargo
            clippy
            rustc
            rustfmt
            rust-analyzer
          ] ++ (with defaultPackage; buildInputs ++ nativeBuildInputs);
        };
      }
    );
}
