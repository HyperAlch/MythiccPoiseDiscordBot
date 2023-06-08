with import <nixpkgs> {};
clangStdenv.mkDerivation {
  name = "rust-sqlless-nix-shell";
  
  nativeBuildInputs = with xorg; [
    pkg-config
  ] ++ [
    python3
  ];

  buildInputs = [ 
    llvm
    openssl
    libxkbcommon
  ];

  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
