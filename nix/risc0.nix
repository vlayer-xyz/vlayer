{ pkgs, system, ... }:

let
  releases = {
    "2.0.0" = {
      "aarch64-darwin" = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-I+SgeXytOSxiXrtyf4aQOdglPKZb0t6GC61Qf4yTn38=";
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
    default = risc0."2.0.0";
    inherit (package "2.0.0") "2.0.0";
    inherit (package "1.2.4") "1.2.4";
  };
}
