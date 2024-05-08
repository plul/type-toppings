{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    basecamp.url = "github:plul/basecamp";
    basecamp.inputs.nixpkgs.follows = "nixpkgs";
    basecamp.inputs.rust-overlay.follows = "rust-overlay";
  };

  outputs =
    inputs@{
      self,
      basecamp,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      flake.basecamp-config = {
        rust.enable = true;
        rust.cargo-udeps.enable = true;
      };

      systems = [ "x86_64-linux" ];

      perSystem =
        { config, pkgs, ... }:
        {
          devShells.default = basecamp.mkShell {
            inherit pkgs;
            config = self.basecamp-config;

            packages = p: [
              p.cargo-outdated
              p.cargo-audit
              p.cargo-hack
            ];
          };
        };
    };
}
