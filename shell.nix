{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    
    # X11 development libraries
    xorg.libX11.dev
    xorg.libXext.dev
    xorg.libXi.dev
    
    # Build tools
    pkg-config
    just
    
    # Testing tools
    xorg.xorgserver  # For Xephyr testing
    imagemagick      # For screenshot comparison
    
    # OpenCV and dependencies for vision grids feature
    opencv4
    clang
    llvm
    libclang
  ];

  # AI.rule: Only add functional commands, not noisy echos
  shellHook = ''
    export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
    export LLVM_CONFIG_PATH="${pkgs.llvm}/bin/llvm-config"
  '';

  # Set environment variables for X11 development
  PKG_CONFIG_PATH = "${pkgs.xorg.libX11.dev}/lib/pkgconfig:${pkgs.xorg.libXext.dev}/lib/pkgconfig:${pkgs.xorg.libXi.dev}/lib/pkgconfig";
}
