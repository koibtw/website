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
      sqlite
      shellcheck
      shfmt
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
  '';
}
