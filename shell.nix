{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
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
    
    # Optional: OpenCV for vision grids (commented out by default)
    # opencv4
  ];

  # AI.rule: Only add functional commands, not noisy echos
  shellHook = ''
  '';

  # Set environment variables for X11 development
  PKG_CONFIG_PATH = "${pkgs.xorg.libX11.dev}/lib/pkgconfig:${pkgs.xorg.libXext.dev}/lib/pkgconfig:${pkgs.xorg.libXi.dev}/lib/pkgconfig";
}
