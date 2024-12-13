{
  description = "vlayer toolbox";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.05";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix";
    risc0.url = "github:kubkon/risc0-flake";
  };


  outputs = { 
    flake-utils, 
    nixpkgs, 
    nixpkgs-unstable,
    foundry, 
    risc0,
    ... 
  } @ inputs: let
    overlays = [ foundry.overlay ] ++ [
      (final: prev: {
        risc0pkgs = risc0.packages.${prev.system};
      })
    ];
    systems = [ "aarch64-darwin" ];
  in 
    flake-utils.lib.eachSystem systems (
      system: let
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        pkgsUnstable = import nixpkgs-unstable {
          inherit system overlays;
        };
        buildInputs = with pkgs; [
          risc0pkgs.rzup
          rustup
          libiconv
          foundry-bin
          pkgsUnstable.bun
          nodejs
          darwin.apple_sdk.frameworks.AppKit # these would not be required if we used rust-overlay... just saying!
          darwin.apple_sdk.frameworks.SystemConfiguration
          darwin.apple_sdk.frameworks.Security
        ];
      in rec {
        devShells.default = pkgs.mkShell {
          name = "vlayer";
          buildInputs = buildInputs;
          CC_riscv32im_risc0_zkvm_elf = "${pkgs.pkgsCross.riscv32-embedded.stdenv.cc}/bin/riscv32-none-elf-cc";
          CFLAGS_riscv32im_risc0_zkvm_elf = "-march=rv32im -nostdlib -DRING_CORE_NOSTDLIBINC=1 -D__ILP32__=1";
          RUST_BACKTRACE = 1;
        };
      }
    );
}
