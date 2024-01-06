let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [ (import rustOverlay) ];
  };

  rust = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };
in
  pkgs.mkShell rec {
    buildInputs = [ rust ] ++ (with pkgs; [
      bacon 
      gcc 
      pkg-config
      rust-analyzer
      stdenv.cc 
      mdsh
    ]);

    LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

    shellHook = ''
        export PS1="''${debian_chroot:+($debian_chroot)}\[\033[01;39m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\W\[\033[00m\]\$ "
        export PS1="(nix-rs)$PS1"
        export LD_LIBRARY_PATH="''${LD_LIBRARY_PATH}:${LD_LIBRARY_PATH}"
    '';
  }
