{ lib
, stdenv
, autoPatchelfHook
, copyDesktopItems
, makeDesktopItem
, makeWrapper
, mkShell
, libxkbcommon
, bacon
, cargo
, clippy
, ffmpeg
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

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  devShell = mkShell {
    packages = [
      bacon
      cargo
      clippy
      ffmpeg
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
  pname = crateName;
  inherit version;

  src = lib.cleanSource ./.;

  inherit cargoDeps;

  nativeBuildInputs = nativeBuildInputs ++ [
    autoPatchelfHook
    copyDesktopItems
    makeWrapper
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

  postFixup = ''
    wrapProgram $out/bin/${crateName} --prefix PATH : ${lib.makeBinPath [
      ffmpeg
    ]}
  '';

  passthru = {
    inherit devShell;
  };
}
