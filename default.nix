{ copyDesktopItems
, glib
, gtk4
, makeDesktopItem
, pkg-config
, naersk
}:

let
  cargoData = builtins.fromTOML (builtins.readFile ./Cargo.toml);
  binaryName = cargoData.package.name;
in

naersk.buildPackage {
  src = ./.;

  buildInputs = [ glib gtk4 ];

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
  ];

  desktopItems = [
    (makeDesktopItem {
      name = binaryName;
      desktopName = builtins.readFile src/title.txt;
      exec = binaryName;
      icon = "video-x-generic";
      terminal = false;
      categories = [ "AudioVideo" "Player" ];
    })
  ];
}
