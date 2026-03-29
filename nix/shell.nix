{
  jq,
  clippy,
  cargo-edit,
  cargo-watch,
  rust-analyzer,
  rustfmt,
  libwebp,
  sqlite,
  shellcheck,
  shfmt,
  imagemagick,
  lutgen,
  whiskers,
  mkShell,
  callPackage,
}:
let
  mainPkg = callPackage ./package.nix { };
  packages = [
    jq
    clippy
    cargo-edit
    cargo-watch
    rust-analyzer
    rustfmt
    libwebp
    sqlite
    shellcheck
    shfmt
    imagemagick
    lutgen
    whiskers
  ]
  ++ (mainPkg.nativeBuildInputs or [ ])
  ++ (mainPkg.buildInputs or [ ]);
in
mkShell {
  inherit packages;
  shellHook = ''
    echo -ne "-----------------------------------\n "
    echo -n "${toString (map (pkg: "• ${pkg.name}\n") packages)}"
    echo "-----------------------------------"
  '';
}
