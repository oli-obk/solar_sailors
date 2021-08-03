let
  pkgs = import <nixpkgs> {}; 
in
pkgs.clangMultiStdenv.mkDerivation rec {
  name = "rustc";
  buildInputs = with pkgs; [
    rustup
    pkg-config
    alsaLib
    libGL
    xorg.libX11
    xorg.libXi
  ];
}

