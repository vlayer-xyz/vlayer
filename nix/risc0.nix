{ pkgs, system, ... }:

let
  releases = {
    "1.2.0" = {
      ${system} = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-0MTqpRoEDVg0O4Iz6wlkTVs7YsvUUmpR0ba3dmzjkhI=";
      };
    };
    "1.1.3" = {
      ${system} = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-AK+E3k6H9oUBNqCMefYc6WtIgrJgxy47CZKa6XGJW0U=";
      };
    };
  };
  artefacts = system: version:
    let
      inherit (releases.${version}.${system}) arch hash;
    in
    pkgs.fetchurl {
      inherit hash;
      url = "https://github.com/risc0/risc0/releases/download/v${version}/cargo-risczero-${arch}.tgz";
    };

  cargo-risczero = version: pkgs.stdenv.mkDerivation {
    name = "cargo-risczero-v${version}";
    version = version;
    src = artefacts system version;
    sourceRoot = ".";
    installPhase = ''
      mkdir -p $out/bin
      cp -r ./cargo-risczero $out/bin/
    '';
  };
  r0vm = version: pkgs.stdenv.mkDerivation {
    name = "risc0-r0vm-v${version}";
    version = version;
    src = artefacts system version;
    sourceRoot = ".";
    installPhase = ''
      mkdir -p $out/bin
      cp -r ./r0vm $out/bin/
    '';
  };
  package = version: {
    ${version} =
      {
        cargo-risczero = cargo-risczero version;
        r0vm = r0vm version;
      };
  };
in
rec {
  risc0 = {
    default = risc0."1.2.0";
    inherit (package "1.2.0") "1.2.0";
    inherit (package "1.1.3") "1.1.3";
  };
}
