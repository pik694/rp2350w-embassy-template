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

        devTools = [
          rustToolchain

          pkgs.cargo-sort
          pkgs.cargo-machete
        ];

        buildInputs = [
          pkgs.flip-link
          pkgs.probe-rs-tools
        ];

        bake-cyw43 = pkgs.writeShellApplication {
          name = "bake-cyw43";
          runtimeInputs = [ ];

          text = ''
            # Find repository root
            REPO_ROOT=$(git rev-parse --show-toplevel)

            ${pkgs.probe-rs-tools}/bin/probe-rs download "$REPO_ROOT/assets/cyw43-firmware/43439A0.bin" \
              --binary-format bin --chip RP235x --base-address 0x10100000 && \
            ${pkgs.probe-rs-tools}/bin/probe-rs download "$REPO_ROOT/assets/cyw43-firmware/43439A0_clm.bin" \
              --binary-format bin --chip RP235x --base-address 0x10140000
          '';
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
