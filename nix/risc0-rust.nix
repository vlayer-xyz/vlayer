{ pkgs, system, ... }:

let
  releases = {
    "1.81.0" = {
      ${system} = {
        arch = "aarch64-apple-darwin";
        hash = "sha256-RhxNjLd1SSs5DqjKk/p/Onr5wpw7MLC2exgffFswEuo=";
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
      mkdir -p $out/lib
      cp -r ./bin $out
      cp -r ./lib $out
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
  risc0-rust = {
    default = risc0-rust."1.81.0";
    inherit (package "1.81.0") "1.81.0";
  };
}
