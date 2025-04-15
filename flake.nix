{
  description = "vlayer toolbox";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix";
  };

  outputs =
    {
      flake-utils,
      nixpkgs,
      nixpkgs-unstable,
      foundry,
      ...
    }:
    let
      systems = [ "aarch64-darwin" ];
    in
    flake-utils.lib.eachSystem systems (
      system:
      let
        overlays = [ foundry.overlay ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgsUnstable = import nixpkgs-unstable {
          inherit system overlays;
        };

        risc0-version = "2.0.1";
        risc0 = (import ./nix/risc0.nix { inherit system pkgs; }).risc0.${risc0-version};

        darwinInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.darwin.apple_sdk.frameworks.Security
          pkgs.darwin.apple_sdk.frameworks.AppKit
        ];

        devTools = [
          risc0.r0vm

          pkgs.foundry-bin

          pkgs.nodejs
          pkgsUnstable.bun

          pkgs.mdbook

          pkgs.cargo-sort
          pkgs.cargo-machete
        ];

        buildInputs = [
          risc0.toolchain

          pkgs.rustup
          pkgs.libiconv
          pkgs.openssl
          pkgs.pkg-config

        ] ++ darwinInputs;

      in
      {

        devShells.default = pkgs.mkShell {
          name = "vlayer-shell";
          buildInputs = buildInputs ++ devTools;

          CC_riscv32im_risc0_zkvm_elf = "${pkgs.pkgsCross.riscv32-embedded.stdenv.cc}/bin/riscv32-none-elf-cc";
          CFLAGS_riscv32im_risc0_zkvm_elf = "-march=rv32im -nostdlib -DRING_CORE_NOSTDLIBINC=1 -D__ILP32__=1";

          CARGO_NET_GIT_FETCH_WITH_CLI = "true";

          RUST_BACKTRACE = 1;
          RUSTFLAGS = "-Dwarnings";

          LIBUSB_STATIC = 1;
        };
      }
    );
}
