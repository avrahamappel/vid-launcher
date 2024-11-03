{ copyDesktopItems
, glib
, gtk3
, makeDesktopItem
, pkg-config
, rustPlatform
}:

let
  cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in

rustPlatform.buildRustPackage rec {
  pname = cargoData.package.name;
  version = cargoData.package.version;

  src = ./.;

  cargoHash = "sha256-UQ44AmD9SG0HrggoSlYzgYnrPZuMZ672U4aNy0dbRSY=";

  buildInputs = [ glib gtk3 ];

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
  ];

  desktopItems = [
    (makeDesktopItem {
      name = pname;
      desktopName = builtins.readFile src/title;
      exec = pname;
      icon = "video-x-generic";
      terminal = false;
      categories = [ "AudioVideo" "Player" ];
    })
  ];
}
