{ copyDesktopItems
, glib
, gtk4
, makeDesktopItem
, pkg-config
, rustPlatform
, version ? null
}:

let
  cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  inherit (cargoData.package) name;
in

rustPlatform.buildRustPackage {
  pname = cargoData.package.name;
  version = if version != null then version else cargoData.package.version;

  src = ./.;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  doCheck = false;

  buildInputs = [ glib gtk4 ];

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
  ];

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
