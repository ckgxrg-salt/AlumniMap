{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchainFor =
          p:
          p.rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

        unfilteredRoot = ./.;
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (
              file:
              lib.any file.hasExt [
                "html"
                "scss"
              ]
            ) unfilteredRoot)
            (lib.fileset.maybeMissing ./frontend/assets)
          ];
        };

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        nativeArgs = commonArgs // {
          pname = "alumnimap";
          cargoExtraArgs = "--bin=alumnimap";
        };

        cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

        backend = craneLib.buildPackage (
          nativeArgs
          // {
            inherit cargoArtifacts;

            preBuild = ''
              mkdir frontend/dist
              cp -r ${frontend}/* frontend/dist
            '';
          }
        );

        wasmArgs = commonArgs // {
          pname = "alumnimap-frontend";
          cargoExtraArgs = "--package=frontend";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        cargoArtifactsWasm = craneLib.buildDepsOnly (
          wasmArgs
          // {
            doCheck = false;
          }
        );

        frontend = craneLib.buildTrunkPackage (
          wasmArgs
          // {
            cargoArtifacts = cargoArtifactsWasm;
            preBuild = ''
              cd ./frontend
            '';
            postBuild = ''
              mv ./dist ..
              cd ..
            '';
            wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
              src = pkgs.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.100";
                hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
              };

              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit src;
                inherit (src) pname version;
                hash = "sha256-qsO12332HSjWCVKtf1cUePWWb9IdYUmT+8OPj/XP2WE=";
              };
            };
          }
        );
      in
      {
        checks = {
          inherit backend frontend;

          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          fmt = craneLib.cargoFmt commonArgs;
        };

        packages = {
          inherit backend frontend;
          default = backend;
        };

        apps.default = flake-utils.lib.mkApp {
          name = "alumnimap";
          drv = backend;
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = with pkgs; [
            trunk
            rustfmt
            clippy
            deadnix
            nixfmt-rfc-style
            sea-orm-cli
            llvmPackages.bintools
          ];
        };

        # A simple database container
        nixosConfigurations."container" = nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            (
              { ... }:
              {
                boot.isContainer = true;
                system.stateVersion = "25.05";
                networking.firewall.allowedTCPPorts = [ 5432 ];
                services.postgresql = {
                  enable = true;
                  enableTCPIP = true;
                  ensureUsers = [
                    {
                      name = "alumnimap";
                      ensureDBOwnership = true;
                    }
                  ];
                  ensureDatabases = [ "alumnimap" ];
                  authentication = ''
                    host alumnimap alumnimap 10.233.1.1/32 trust
                  '';
                };
                users.users."alumnimap" = {
                  isSystemUser = true;
                  group = "alumnimap";
                };
                users.groups."alumnimap" = { };
              }
            )
          ];
        };
      }
    );
}
