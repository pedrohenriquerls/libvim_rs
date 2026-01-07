let
  nixpkgs-src = builtins.fetchTarball {
    # 23.05
    url = "https://github.com/NixOS/nixpkgs/archive/nixos-25.11.tar.gz";
  };

  pkgs = import nixpkgs-src {
    config = {
      # allowUnfree may be necessary for some packages, but in general you should not need it.
      allowUnfree = false;
    };
  };

  lib = pkgs.lib;
  stdenv = pkgs.stdenv;

  lib-path = with pkgs; lib.makeLibraryPath [ libffi openssl ];

  shell = pkgs.mkShell {
    buildInputs = with pkgs; [
      # other packages needed for compiling python libs
      nasm
      xorg.libXt
      xorg.libSM
      xorg.libICE
      xorg.libXxf86vm
      xorg.libxkbfile
      nodePackages.node-gyp

      gcc
      gnumake
      fetchutils
      ncurses5
      rustup
      nodePackages.eas-cli
      bash
      readline
      libffi
      openssl
      llvmPackages.libcxxStdenv
      clang

      # unfortunately needed because of messing with LD_LIBRARY_PATH below
      git
      openssh
      rsync
    ]
    # Conditionally add glibc only on Linux (not available on Darwin)
    ++ lib.optional stdenv.isLinux pkgs.glibc
    # Conditionally add Darwin-specific dependencies
    # Note: libiconv is included in the SDK by default, but we add it explicitly
    # if the iconv binary is needed. Frameworks like Security are available
    # through the default SDK automatically.
    ++ lib.optional stdenv.isDarwin pkgs.libiconv;

    shellHook = ''
      rustup update

      # Augment the dynamic linker path
      export "LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${lib-path}"
      export "LIBCLANG_PATH=${pkgs.libclang.lib}/lib";
    '';
  };

in shell
