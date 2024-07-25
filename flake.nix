{
  description = "Wallust, a better pywal";

  # Nixpkgs / NixOS version to use.
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-parts,
    flake-compat,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      # can be extended, but these have proper binary cache support in nixpkgs
      # as of writing.
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = {
        self',
        config,
        pkgs,
        ...
      }: let
        toolchain = inputs.fenix.packages.${pkgs.system}.minimal.toolchain;
      in {
        packages.default = config.packages.wallust;
        packages.wallust = pkgs.callPackage ./build.nix {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        };

        formatter = pkgs.alejandra;

        devShells.default = pkgs.mkShell {
          inputsFrom = [config.packages.default];
        };
      };
    };

  # Allows the user to use nix-community cache when using `nix run <thisFlake>`.
  nixConfig = {
    extra-substituters = ["https://nix-community.cachix.org"];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };
}
