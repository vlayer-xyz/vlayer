{ pkgs, system, ... }:

let
  releases = {
    "2.0.1" = {
      "aarch64-darwin" = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-d/gQVXK+WAPTGRnx8Fw3XTWe6oKPw8FsLK7VNxC9aF8=";
      };
    };
    "1.2.4" = {
      "aarch64-darwin" = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-0MTqpRoEDVg0O4Iz6wlkTVs7YsvUUmpR0ba3dmzjkhI=";
      };
    };
  };
  artifacts =
    system: version:
    let
      inherit (releases.${version}.${system}) arch hash;
    in
    pkgs.fetchurl {
      inherit hash;
      url = "https://github.com/risc0/risc0/releases/download/v${version}/cargo-risczero-${arch}.tgz";
    };

  toolchain =
    version:
    pkgs.stdenv.mkDerivation {
      name = "cargo-risczero-v${version}";
      version = version;
      src = artifacts system version;
      sourceRoot = ".";
      installPhase = ''
        mkdir -p $out/bin
        cp -r ./cargo-risczero $out/bin/
      '';
    };
  r0vm =
    version:
    pkgs.stdenv.mkDerivation {
      name = "risc0-r0vm-v${version}";
      version = version;
      src = artifacts system version;
      sourceRoot = ".";
      installPhase = ''
        mkdir -p $out/bin
        cp -r ./r0vm $out/bin/
      '';
    };
  package = version: {
    ${version} = {
      toolchain = toolchain version;
      r0vm = r0vm version;
    };
  };
in
rec {
  risc0 = {
    default = risc0."2.0.1";
    inherit (package "2.0.1") "2.0.1";
    inherit (package "1.2.4") "1.2.4";
  };
}
