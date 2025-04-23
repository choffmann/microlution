{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        # Read the file relative to the flake's root
        overrides = builtins.fromTOML (builtins.readFile (self + "/rust-toolchain.toml"));
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildBuild.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        buildInputs = with pkgs; [openssl];
        nativeBuildInputs = with pkgs; [
          # rustup
          rustToolchain
          pkg-config
          lld
          sqlx-cli
          cargo-watch
          cargo-expand
          cargo-udeps
        ];
      in {
        devShells.default = pkgs.mkShell rec {
          inherit buildInputs nativeBuildInputs;

          # RUSTC_VERSION = overrides.toolchain.channel;

          # shellHook = ''
          #   export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          #   export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          # '';

          # nativeBuildInputs = [pkgs.pkg-config];
          # buildInputs = with pkgs; [
          #   rust-analyzer
          #   rustToolchain
          #   clang
          #   lld
          #   llvmPackages.bintools
          #   openssl
          #
          #   sqlx-cli
          #   cargo-watch
          #   cargo-expand
          # ];

          # RUSTC_VERSION = overrides.toolchain.channel;
          #
          # # https://github.com/rust-lang/rust-bindgen#environment-variables
          # LIBCLANG_PATH = pkgs.lib.makeLibraryPath [pkgs.llvmPackages_latest.libclang.lib];
          #
          # shellHook = ''
          #   export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          #   export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          # '';
          #
          # # Add precompiled library to rustc search path
          # RUSTFLAGS = builtins.map (a: ''-L ${a}/lib'') [
          #   # add libraries here (e.g. pkgs.libvmi)
          # ];
          #
          # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
          #
          # # Add glibc, clang, glib, and other headers to bindgen search path
          # BINDGEN_EXTRA_CLANG_ARGS =
          #   # Includes normal include path
          #   (builtins.map (a: ''-I"${a}/include"'') [
          #     # add dev libraries here (e.g. pkgs.libvmi.dev)
          #     pkgs.glibc.dev
          #   ])
          #   # Includes with special directory paths
          #   ++ [
          #     ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
          #     ''-I"${pkgs.glib.dev}/include/glib-2.0"''
          #     ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
          #   ];
        };
      }
    );
}
