{
  description = "my personal website :3";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    whiskers = {
      url = "git+https://codeberg.org/evergarden/whiskers.git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      whiskers,
    }:
    let
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      forAllSystems =
        f:
        nixpkgs.lib.genAttrs systems (
          system:
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ whiskers.overlays.default ];
            };
          in
          f pkgs
        );
    in
    {
      packages = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/package.nix { };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix { };
      });

      formatter = forAllSystems (pkgs: pkgs.callPackage ./nix/formatter.nix { });
    };
}
