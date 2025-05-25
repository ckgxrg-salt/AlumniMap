{
  rustPlatform,
  trunk,
  makeWrapper,
  llvmPackages,
  openssl,
  pkg-config,
  libxkbcommon,
  libGL,
  fontconfig,
  wayland,
  xorg,
  wasm-bindgen-cli,
  binaryen,
}:
rustPlatform.buildRustPackage {
  pname = "alumnimap";
  version = "0.1.0";

  src = ./.;
  cargoHash = "sha256-5Wj7YUM/3mva1MvcCa/ai6qr9rhRq70ng2Wb9vG8ibc=";

  nativeBuildInputs = [
    trunk
    makeWrapper
    llvmPackages.bintools
    wasm-bindgen-cli
    binaryen
  ];
  buindInputs = [
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

  buildPhase = ''
    export HOME=$(mktemp -d)
    mkdir -p $HOME/.cargo $HOME/.cache
    export CARGO_HOME=$HOME/.cargo
    export TRUNK_CACHE=$HOME/.cache/trunk

    cd frontend
    trunk build --release

    cd ../backend
    cargo build --release
  '';

  installPhase = ''
    cd ..

    mkdir -p $out/share
    cp -r frontend/dist $out/share/dist

    mkdir -p $out/bin
    cp target/release/alumnimap $out/bin
    wrapProgram $out/bin/alumnimap --set DIST_DIR "$out/share/dist"
  '';
}
