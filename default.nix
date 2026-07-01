{ lib
, stdenv
, autoPatchelfHook
, copyDesktopItems
, makeDesktopItem
, libxkbcommon
, bacon
, clippy
, rustfmt
, rust-analyzer
, vulkan-loader
, wayland
, pkg-config
, craneLib
, version ? null
}:

let
  cargoData = fromTOML (builtins.readFile ./Cargo.toml);
  crateName = cargoData.package.name;

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

  common = {
    pname = cargoData.package.name;
    version = if version != null then version else cargoData.package.version;

    src = craneLib.cleanCargoSource ./.;
  };

  cargoArtifacts = craneLib.buildDepsOnly common;

  devShell = craneLib.devShell {
    packages = [
      bacon
      clippy
      rustfmt
      rust-analyzer
    ] ++ nativeBuildInputs;

    LD_LIBRARY_PATH = dlopenLibraryPath;
    RUST_BACKTRACE = 1;
    VID_LAUNCHER_TITLE = "${title} (DEBUG)";
  };
in

craneLib.buildPackage common // {
  inherit cargoArtifacts;

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
