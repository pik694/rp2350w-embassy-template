{
  description = "rp2350w-embassy-template";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };
  outputs =
    {
      flake-utils,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        buildInputs = [
          rustToolchain
        ];

        devTools = [
          pkgs.cargo-sort
          pkgs.cargo-machete

          pkgs.flip-link
          pkgs.probe-rs-tools
        ];

        bake-cyw43 = pkgs.writeShellApplication {
          name = "bake-cyw43";
          runtimeInputs = [ pkgs.git ] ++ devTools;
          text = builtins.readFile ./scripts/bake-cyw43.sh;
        };

        cargo-lint = pkgs.writeShellApplication {
          name = "cargo-lint";
          runtimeInputs = devTools;
          text = builtins.readFile ./scripts/lint.sh;
        };

      in
      {
        packages = {
          inherit bake-cyw43 cargo-lint;
        };

        apps = {
          bake-cyw43 = {
            type = "app";
            program = "${bake-cyw43}/bin/bake-cyw43";
          };
          cargo-lint = {
            type = "app";
            program = "${cargo-lint}/bin/cargo-lint";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs =
            buildInputs
            ++ devTools
            ++ [
              bake-cyw43
              cargo-lint
            ];
        };
      }
    );

}
