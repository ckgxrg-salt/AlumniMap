{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
    in
    {
      packages.${system}.default = pkgs.callPackage ./package.nix { };
      devShells.${system}.default = pkgs.mkShell rec {
        name = "alumnimap";

        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            targets = [ "wasm32-unknown-unknown" ];
          })

          clippy
          rustfmt
          deadnix
          nixfmt-rfc-style

          sea-orm-cli

          trunk
          llvmPackages.bintools
        ];

        buildInputs = with pkgs; [
          openssl
          pkg-config
          libxkbcommon
          libGL
          fontconfig
          wayland
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
        ];
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
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
    };
}
