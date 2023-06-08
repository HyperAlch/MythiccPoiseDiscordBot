with import <nixpkgs> {};
clangStdenv.mkDerivation {
  name = "rust-sqlless-nix-shell";
  
  nativeBuildInputs = with xorg; [
    libxcb
    libXcursor
    libXrandr
    libXi
    pkg-config
  ] ++ [
    python3
    libGL
    libGLU
  ];

  buildInputs = [ 
    llvm
    xorg.libX11
    wayland
    trunk
    openssl
    nodePackages.tailwindcss
    libxkbcommon
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  shellHook = ''
      export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU])}
  '';
}
