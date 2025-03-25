{
  description = "vlayer-x86_64-unknown-linux-musl";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      target = "x86_64-unknown-linux-musl";
      system = "x86_64-linux";
    in
    {
      devShells.${system}.default =
        with nixpkgs.legacyPackages.${system};
        mkShell {
          name = "vlayer-x86_64-unknown-linux-musl";

          nativeBuildInputs = [
            bintools
            clang
          ];

          buildInputs = with pkgsCross.musl64.pkgsStatic; [
            openssl
            libz
          ];

          TARGET_CC = "${pkgsCross.musl64.pkgsStatic.stdenv.cc}/bin/${pkgsCross.musl64.pkgsStatic.stdenv.cc.targetPrefix}cc";
          OPENSSL_STATIC = "1";
          OPENSSL_INCLUDE_DIR = "${pkgsCross.musl64.pkgsStatic.openssl.dev}/include";
          OPENSSL_LIB_DIR = "${pkgsCross.musl64.pkgsStatic.openssl.out}/lib";
          CARGO_BUILD_TARGET = target;
          CARGO_BUILD_RUSTFLAGS = [
            "-C"
            "target-feature=+crt-static"
            "-C"
            "link-args=-static"
            "-C"
            "linker=${pkgsCross.musl64.pkgsStatic.stdenv.cc}/bin/${pkgsCross.musl64.pkgsStatic.stdenv.cc.targetPrefix}cc"
          ];
          LIBCLANG_PATH = lib.makeLibraryPath [ llvmPackages_latest.libclang.lib ];
          BINDGEN_EXTRA_CLANG_ARGS =
            (builtins.map (a: ''-I"${a}/include"'') [
              pkgs.glibc.dev
            ])
            ++ [
              ''-I"${llvmPackages_latest.libclang.lib}/lib/clang/${llvmPackages_latest.libclang.version}/include"''
              ''-I"${glib.dev}/include/glib-2.0"''
              ''-I${glib.out}/lib/glib-2.0/include/''
            ];
        };
    };
}
