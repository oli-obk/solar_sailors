let
  pkgs = import <nixpkgs> {}; 
in
pkgs.mkShell rec {
  name = "rustc";
  buildInputs = with pkgs; [
    cargo
    rustfmt
    pkg-config
    alsaLib
    libGL
    xorg.libX11
    xorg.libXi
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}

