{
  description = "vlayer toolbox";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix";
    risc0.url = "github:kubkon/risc0-flake";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { 
    flake-utils, 
    nixpkgs, 
    nixpkgs-unstable,
    foundry, 
    risc0,
    rust-overlay,
    crane,
    ... 
  } @ inputs: let
    overlays = [ foundry.overlay ] ++ [
      (final: prev: {
        risc0pkgs = risc0.packages.${prev.system};
      })
    ] ++ [(import rust-overlay)];
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

        toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        commonBuildInputs = with pkgs; [
          rustup
          foundry-bin
          libiconv
        ];

        buildInputs = commonBuildInputs ++ (with pkgs; [
          risc0pkgs.rzup
          pkgsUnstable.bun
          nodejs
          darwin.apple_sdk.frameworks.AppKit # these would not be required if we used rust-overlay... just saying!
          darwin.apple_sdk.frameworks.SystemConfiguration
          darwin.apple_sdk.frameworks.Security
        ]);

        CC_riscv32im_risc0_zkvm_elf = "${pkgs.pkgsCross.riscv32-embedded.stdenv.cc}/bin/riscv32-none-elf-cc";
        CFLAGS_riscv32im_risc0_zkvm_elf = "-march=rv32im -nostdlib -DRING_CORE_NOSTDLIBINC=1 -D__ILP32__=1";
      in rec {
        devShells.default = pkgs.mkShell {
          name = "vlayer";
          buildInputs = buildInputs;

          CC_riscv32im_risc0_zkvm_elf = CC_riscv32im_risc0_zkvm_elf;
          CFLAGS_riscv32im_risc0_zkvm_elf = CFLAGS_riscv32im_risc0_zkvm_elf;

          RUST_BACKTRACE = 1;
        };

        packages.guest-wrappers = craneLib.buildPackage (with pkgs; {
          pname = "guest-wrappers";
          version = "0.0.0";
          src = craneLib.cleanCargoSource ./.;

          buildInputs = commonBuildInputs ++ [
            pkg-config
            darwin.DarwinTools
          ];

          doCheck = false;

          postUnpack = ''
            cd $sourceRoot/rust
            sourceRoot="."
          '';
          cargoToml = ./rust/Cargo.toml;
          cargoLock = ./rust/Cargo.lock;

          # preBuild = ''
          # expo
          # '';

          cargoVendorDir = craneLib.vendorCargoDeps {
            src = craneLib.cleanCargoSource ./rust;

            overrideVendorCargoPackage = p: drv:
              if p.name == "risc0-build" then
                drv.overrideAttrs (_old: {
                  patches = [
                    ./nix-patches/0001-patch-risc0-build.patch
                  ];
                })
              else
                drv;
          };

          cargoBuildCommand = "cargo build --release -p call_guest_wrapper";

          CC_riscv32im_risc0_zkvm_elf = CC_riscv32im_risc0_zkvm_elf;
          CFLAGS_riscv32im_risc0_zkvm_elf = CFLAGS_riscv32im_risc0_zkvm_elf;

          RISC0_TOOLCHAIN_PATH = "$HOME/.risc0/toolchains/rust_aarch64-apple-darwin_r0.1.81.0";
        });
      }
    );
}
