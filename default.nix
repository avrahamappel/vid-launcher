{ lib
, copyDesktopItems
, makeDesktopItem
, mkShell
, libxkbcommon
, bacon
, cargo
, clippy
, rustc
, rustfmt
, rust-analyzer
, vulkan-loader
, wayland
, pkg-config
, rustPlatform
, version ? null
}:

let
  cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  inherit (cargoData.package) name;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
  ];

  dlopenLibraries = [
    libxkbcommon
    vulkan-loader
    wayland
  ];

  RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lib.makeLibraryPath dlopenLibraries}";

  devShell = mkShell {
    packages = [
      bacon
      cargo
      clippy
      rustc
      rustfmt
      rust-analyzer
    ] ++ nativeBuildInputs;

    inherit RUSTFLAGS;
    RUST_BACKTRACE = 1;
  };
in

rustPlatform.buildRustPackage {
  pname = cargoData.package.name;
  version = if version != null then version else cargoData.package.version;

  src = ./.;

  inherit cargoDeps;

  nativeBuildInputs = nativeBuildInputs ++ [ copyDesktopItems ];

  inherit RUSTFLAGS;

  desktopItems = [
    (makeDesktopItem {
      name = name;
      desktopName = builtins.readFile src/title.txt;
      exec = name;
      icon = "video-x-generic";
      terminal = false;
      categories = [ "AudioVideo" "Player" ];
    })
  ];

  passthru = {
    inherit devShell;
  };
}
