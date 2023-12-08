let
  nixpkgs-src = builtins.fetchTarball {
    # 23.05
    url = "https://github.com/NixOS/nixpkgs/archive/nixos-23.05.tar.gz";
  };

  pkgs = import nixpkgs-src {
    config = {
      # allowUnfree may be necessary for some packages, but in general you should not need it.
      allowUnfree = false;
    };
  };

  lib-path = with pkgs; lib.makeLibraryPath [ libffi openssl ];

  shell = pkgs.mkShell {
    buildInputs = [
      # other packages needed for compiling python libs
      pkgs.nasm
      pkgs.xorg.libXt
      pkgs.xorg.libSM
      pkgs.xorg.libICE
      pkgs.xorg.libXxf86vm
      pkgs.xorg.libxkbfile
      pkgs.nodePackages.node-gyp

      pkgs.gcc
      pkgs.glibc
      pkgs.gnumake
      pkgs.fetchutils
      pkgs.ncurses5
      pkgs.rustup
      pkgs.nodePackages.esy
      pkgs.nodePackages.eas-cli
      pkgs.bash
      pkgs.readline
      pkgs.libffi
      pkgs.openssl
      pkgs.llvmPackages.libcxxStdenv
      pkgs.clang

      # unfortunately needed because of messing with LD_LIBRARY_PATH below
      pkgs.git
      pkgs.openssh
      pkgs.rsync
    ];

    shellHook = ''
      rustup update
    '';
  };

in shell
