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

  cargoHash = "sha256-Mexo+7WRohotYoH9sDEaG6yxhHvIRixmtDZ03Ogwhog=";

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
