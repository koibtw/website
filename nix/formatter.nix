{
  treefmt,
  nixfmt,
  rustfmt,
  prettier,
  shfmt,
}:
treefmt.withConfig {
  runtimeInputs = [
    nixfmt
    rustfmt
    prettier
    shfmt
  ];
  settings = {
    on-unmatched = "info";
    tree-root-file = "flake.nix";
    formatter = {
      nixfmt = {
        command = "nixfmt";
        includes = [ "*.nix" ];
      };
      rustfmt = {
        command = "rustfmt";
        options = [
          "--edition"
          "2024"
        ];
        includes = [ "*.rs" ];
      };
      prettier = {
        command = "prettier";
        options = [ "--write" ];
        includes = [
          "*.css"
          "*.js"
          "*.json"
        ];
      };
      shfmt = {
        command = "shfmt";
        options = [ "-w" ];
        includes = [
          "*.sh"
          "*.bash"
        ];
      };
    };
  };
}
