let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };

  # If nixos-unstable is missing
  ## `sudo nix-channel --add https://nixos.org/channels/nixos-unstable nixos-unstable`
  ## `sudo nix-channel --update nixos-unstable`
  unstable = import <nixos-unstable> {};
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
  pkgs.mkShell {
    packages = [
      toolchain
      pkgs.rust-analyzer-unwrapped
      unstable.sqlx-cli
      pkgs.sqlitebrowser
    ];
    
 
  buildInputs = [ 
    pkgs.openssl
    pkgs.pkg-config
    pkgs.bash
  ];
    RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
  }
  
  
