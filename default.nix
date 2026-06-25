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
  cargoData = fromTOML (builtins.readFile ./Cargo.toml);
  crateName = cargoData.package.name;

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

  title = "Random Vid Launcher";

  env = {
    RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lib.makeLibraryPath dlopenLibraries}";
    VID_LAUNCHER_TITLE = title;
  };

  devShell = mkShell {
    packages = [
      bacon
      cargo
      clippy
      rustc
      rustfmt
      rust-analyzer
    ] ++ nativeBuildInputs;

    inherit env;

    RUST_BACKTRACE = 1;
  };
in

rustPlatform.buildRustPackage {
  pname = cargoData.package.name;
  version = if version != null then version else cargoData.package.version;

  src = ./.;

  inherit cargoDeps env;

  nativeBuildInputs = nativeBuildInputs ++ [ copyDesktopItems ];

  desktopItems = [
    (makeDesktopItem {
      name = crateName;
      desktopName = title;
      exec = crateName;
      icon = "video-x-generic";
      terminal = false;
      categories = [ "AudioVideo" "Player" ];
    })
  ];

  passthru = {
    inherit devShell;
  };
}
