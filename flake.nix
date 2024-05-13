{
  description = "Wallust, a better pywal";

  # Nixpkgs / NixOS version to use.
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.utils.url = "github:numtide/flake-utils";
  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    {
      overlays.default = final: prev: {
        wallust = final.callPackage ./build.nix {};
      };
    }
    // utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [self.overlays.default];
      };
    in {
      packages = {
        inherit (pkgs) wallust;
        default = pkgs.wallust;
      };

      formatter = pkgs.alejandra;

      devShells.default = pkgs.callPackage ({
        mkShell,
        rustc,
        cargo,
        gnumake,
        pkg-config,
        man,
        imagemagick,
        wallust,
      }:
        mkShell {
          inherit (wallust) nativeBuildInputs buildInputs;
        }) {};
    });
}
