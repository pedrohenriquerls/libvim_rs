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
      pkgs.gnumake
      pkgs.fetchutils
      pkgs.ncurses5
      pkgs.rustup
      pkgs.nodePackages.esy
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
      # Augment the dynamic linker path
      export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib-path}"
      export "LIBCLANG_PATH=${pkgs.libclang.lib}/lib";
    '';
  };

in shell
