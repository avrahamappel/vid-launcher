{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:/ipetkov/crane";
  };

  outputs =
    { self
    , flake-utils
    , nixpkgs
    , crane
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
          craneLib = crane.mkLib pkgs;
          version = builtins.concatStringsSep "-" [
            (builtins.substring 0 4 self.lastModifiedDate)
            (builtins.substring 4 2 self.lastModifiedDate)
            (builtins.substring 6 2 self.lastModifiedDate)
            (self.shortRev or self.dirtyShortRev)
          ];
        };

        inherit (defaultPackage) devShell;
      }
    );
}
