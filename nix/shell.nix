{ pkgs }:

let
  mainPkg = if builtins.pathExists ./default.nix then pkgs.callPackage ./default.nix { } else { };

  pkgInputs =
    with pkgs;
    [
      jq
      clippy
      cargo-edit
      cargo-watch
      cargo-shuttle
      rust-analyzer
      rustfmt
      libwebp
      docker
      postgresql
    ]
    ++ (mainPkg.nativeBuildInputs or [ ])
    ++ (mainPkg.buildInputs or [ ]);
in
pkgs.mkShell {
  packages = pkgInputs;

  shellHook = ''
    echo -ne "-----------------------------------\n "

    echo -n "${toString (map (pkg: "• ${pkg.name}\n") pkgInputs)}"

    echo "-----------------------------------"

    export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/docker.sock
    echo "expecting rootless dockerd at $DOCKER_HOST"
  '';
}
