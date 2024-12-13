{ pkgs, system, ... }:

let
  releases = {
    "1.81.0" = {
      ${system} = {
        arch = "aarch64-apple-darwin";
        hash = pkgs.lib.fakeHash;
      };
    };
  };
  artefacts = system: version:
    let
      inherit (releases.${version}.${system}) arch hash;
    in
    pkgs.fetchurl {
      inherit hash;
      url = "https://github.com/risc0/rust/releases/download/r0.${version}/rust-toolchain-${arch}.tar.gz";
    };

  toolchain = version: pkgs.stdenv.mkDerivation {
    name = "risc0-rust-toolchain-r0.${version}";
    version = version;
    src = artefacts system version;
    sourceRoot = ".";
    installPhase = ''
      mkdir -p $out/bin
      cp -r ./bin $out/bin/
      mkdir -p $out/lib
      cp -r ./lib $out/lib/
    '';
  };
  package = version: {
    ${version} =
      {
        toolchain = toolchain version;
      };
  };
in
rec {
  risc0-rust-toolchain = {
    default = risc0-rust-toolchain."1.81.0";
    inherit (package "1.81.0") "1.81.0";
  };
}
