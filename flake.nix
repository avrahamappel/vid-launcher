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
          version = builtins.concatStringsSep "-" [
            (builtins.substring 0 4 self.lastModifiedDate)
            (builtins.substring 4 2 self.lastModifiedDate)
            (builtins.substring 6 2 self.lastModifiedDate)
            (self.shortRev or self.dirtyShortRev)
          ];
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
