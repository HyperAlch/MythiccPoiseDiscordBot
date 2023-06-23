let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
  pkgs.mkShell {
    packages = [
      toolchain
      pkgs.rust-analyzer-unwrapped
    ];
    
  # For building `cargo-shuttle`
  buildInputs = [ 
    pkgs.openssl
    pkgs.pkg-config
  ];

    
    RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

    # `mkdir temp` if temp directory is missing
    TMPDIR = "${builtins.toString ./.}/temp";

    # `cargo install cargo-shuttle` if shuttle has been garbage collected
  }
  
  
