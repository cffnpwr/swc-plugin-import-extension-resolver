{
  description = "A SWC plugin to resolve import extensions.";

  inputs = {
    nixpkgs = { url = "github:NixOS/nixpkgs/nixos-unstable"; };
    flake-utils = { url = "github:numtide/flake-utils"; };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, flake-compat, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs.lib) optional;
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            cargo
            rust-analyzer
            nodePackages.pnpm
          ];
          shellHook = ''
            rustup override set nightly
            rustup target add wasm32-unknown-unknown
            rustup target add wasm32-wasi
          '';
        };
      });
}
