{
  description = "A Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk/master";
    fenix.url = "github:nix-community/fenix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
    fenix,
    rust-overlay,
    ...
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
              self.overlays.default
            ];
          };
        });

    buildTargets = {
      "x86_64-linux" = {
        crossSystemConfig = "x86_64-unknown-linux-musl";
        rustTarget = "x86_64-unknown-linux-musl";
      };

      "aarch64-linux" = {
        crossSystemConfig = "aarch64-unknown-linux-musl";
        rustTarget = "aarch64-unknown-linux-musl";
      };

      # Old Raspberry Pi's
      "armv6l-linux" = {
        crossSystemConfig = "armv6l-unknown-linux-musleabihf";
        rustTarget = "arm-unknown-linux-musleabihf";
      };
    };

    eachSystem = supportedSystems: callback:
      builtins.foldl'
      (overall: system: overall // {${system} = callback system;})
      {}
      supportedSystems;

    eachCrossSystem = supportedSystems: callback:
      eachSystem supportedSystems (
        buildSystem:
          builtins.foldl'
          (inner: targetSystem:
            inner
            // {
              "cross-${targetSystem}" = callback buildSystem targetSystem;
            })
          {default = callback buildSystem buildSystem;}
          supportedSystems
      );

    mkPkgs = buildSystem: targetSystem:
      import nixpkgs ({
          system = buildSystem;
        }
        // (
          if targetSystem == null
          then {}
          else {
            # The nixpkgs cache doesn't have any packages where cross-compiling has
            # been enabled, even if the target platform is actually the same as the
            # build platform (and therefore it's not really cross-compiling). So we
            # only set up the cross-compiling config if the target platform is
            # different.
            crossSystem.config = buildTargets.${targetSystem}.crossSystemConfig;
          }
        ));
  in {
    overlays.default = final: prev: {
      rustToolchain = final.rust-bin.stable.latest.default.override {
        extensions = ["rust-src"];
        targets = ["aarch64-unknown-linux-gnu"];
      };
    };

    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer

          SDL2
        ];

        env = {
          # Required by rust-analyzer
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });

    packages =
      eachCrossSystem
      (builtins.attrNames buildTargets)
      (
        buildSystem: targetSystem: let
          pkgs = mkPkgs buildSystem null;
          pkgsCross = mkPkgs buildSystem targetSystem;
          rustTarget = buildTargets.${targetSystem}.rustTarget;

          fenixPkgs = fenix.packages.${buildSystem};

          mkToolchain = fenixPkgs:
            fenixPkgs.toolchainOf {
              channel = "stable";
              sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
            };

          toolchain = fenixPkgs.combine [
            (mkToolchain fenixPkgs).rustc
            (mkToolchain fenixPkgs).cargo
            (mkToolchain fenixPkgs.targets.${rustTarget}).rust-std
          ];

          buildPackageAttrs =
            if builtins.hasAttr "makeBuildPackageAttrs" buildTargets.${targetSystem}
            then buildTargets.${targetSystem}.makeBuildPackageAttrs pkgsCross
            else {};

          naersk-lib = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
          naersk-lib.buildPackage (buildPackageAttrs
            // rec {
              src = ./.;
              strictDeps = true;
              doCheck = false;

              nativeBuildInputs = [
                pkgs.perl
              ];

              OPENSSL_STATIC = "1";
              OPENSSL_LIB_DIR = "${pkgsCross.pkgsStatic.openssl.out}/lib";
              OPENSSL_INCLUDE_DIR = "${pkgsCross.pkgsStatic.openssl.dev}/include";
              TARGET_CC = "${pkgsCross.stdenv.cc}/bin/${pkgsCross.stdenv.cc.targetPrefix}cc";

              CARGO_BUILD_TARGET = rustTarget;
              CARGO_BUILD_RUSTFLAGS = [
                "-C"
                "target-feature=+crt-static"

                "-C"
                "link-args=-static -latomic"

                # https://github.com/rust-lang/cargo/issues/4133
                "-C"
                "linker=${TARGET_CC}"
              ];
            })
      );
  };
}
