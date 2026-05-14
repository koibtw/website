{
  lib,
  rustPlatform,
  just,
  whiskers,
}:

let
  p = (lib.importTOML ../Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = p.name;
  inherit (p) version;

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    just
    whiskers
  ];

  dontUseJustInstall = true;
  dontUseJustBuild = true;
  dontUseJustCheck = true;

  preBuild = ''
    just styles
  '';

  doCheck = false;

  meta = {
    inherit (p) description homepage;
    license = [
      "The Happy Bunny License"
      lib.licenses.mit
    ];
    maintainers = with lib.maintainers; [ koi ];
    mainProgram = "kitget";
  };
}
