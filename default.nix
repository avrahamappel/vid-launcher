{ lib
, copyDesktopItems
, makeDesktopItem
, libxkbcommon
, vulkan-loader
, wayland
, pkg-config
, rustPlatform
, version ? null
}:

let
  cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  inherit (cargoData.package) name;

  dlopenLibraries = [
    libxkbcommon
    vulkan-loader
    wayland
  ];
in

rustPlatform.buildRustPackage {
  pname = cargoData.package.name;
  version = if version != null then version else cargoData.package.version;

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  buildInputs = [ ];

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
  ];

  env.RUSTFLAGS = "-C link-arg=-Wl,-rpath,${lib.makeLibraryPath dlopenLibraries}";

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
}
