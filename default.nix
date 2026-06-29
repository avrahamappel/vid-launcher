{ lib
, stdenv
, autoPatchelfHook
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

  dlopenLibraryPath = lib.makeLibraryPath dlopenLibraries;

  title = "Random Vid Launcher";

  devShell = mkShell {
    packages = [
      bacon
      cargo
      clippy
      rustc
      rustfmt
      rust-analyzer
    ] ++ nativeBuildInputs;

    LD_LIBRARY_PATH = dlopenLibraryPath;
    RUST_BACKTRACE = 1;
    VID_LAUNCHER_TITLE = "${title} (DEBUG)";
  };
in

rustPlatform.buildRustPackage {
  pname = cargoData.package.name;
  version = if version != null then version else cargoData.package.version;

  src = lib.cleanSource ./.;

  inherit cargoDeps;

  nativeBuildInputs = nativeBuildInputs ++ [
    autoPatchelfHook
    copyDesktopItems
  ];

  appendRunpaths = [ dlopenLibraryPath ];

  buildInputs = [
    libxkbcommon
    vulkan-loader
    wayland
    stdenv.cc.cc
  ];

  # RUSTFLAGS = "-C link-arg=-Wl,-rpath,${dlopenLibraryPath}";
  VID_LAUNCHER_TITLE = title;

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
