{
  description = "vlayer toolbox";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-24.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    foundry.url = "github:shazow/foundry.nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { 
    flake-utils, 
    nixpkgs, 
    nixpkgs-unstable,
    foundry, 
    rust-overlay,
    crane,
    ... 
  } @ inputs: let
    overlays = [ foundry.overlay ] ++ [(import rust-overlay)];
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

        risc0-version = "1.2.0";
        risc0 = (import ./nix/risc0.nix { inherit system pkgs; }).risc0.${risc0-version};

        risc0-rust-version = "1.81.0";
        risc0-rust = (import ./nix/risc0-rust.nix { inherit system pkgs; }).risc0-rust.${risc0-rust-version};

        commonBuildInputs = with pkgs; [
          risc0-rust.toolchain
          foundry-bin
          libiconv
        ];

        buildInputs = commonBuildInputs ++ (with pkgs; [
          risc0.cargo-risczero
          rustup
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
          src = ./.;

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
            src = ./.;
            cargoLock = ./rust/Cargo.lock;

            overrideVendorCargoPackage = p: drv:
              if p.name == "risc0-build" then
                drv.overrideAttrs (_old: {
                  patches = [
                    ./nix/patches/0001-patch-risc0-build.patch
                  ];
                })
              else
                drv;
          };

          cargoBuildCommand = "cargo build --release -p call_guest_wrapper";

          CC_riscv32im_risc0_zkvm_elf = CC_riscv32im_risc0_zkvm_elf;
          CFLAGS_riscv32im_risc0_zkvm_elf = CFLAGS_riscv32im_risc0_zkvm_elf;

          RISC0_TOOLCHAIN_PATH = "${risc0-rust.toolchain.out}";
        });
      }
    );
}
