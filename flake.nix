{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        name = "alumnimap";

        buildInputs = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
          deadnix
          nixfmt-rfc-style

          # Backend
          loco
          sea-orm-cli

          # Frontend
          trunk
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
    };
}
