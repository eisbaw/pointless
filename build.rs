use pkg_config::Config;

fn main() {
    // Use pkg-config to find X11 libraries - fail fast if not available
    // The Nix environment provides these via PKG_CONFIG_PATH

    Config::new()
        .atleast_version("1.6")
        .probe("x11")
        .unwrap_or_else(|e| {
            panic!(
                "X11 development libraries not found: {}.\nRun: nix develop",
                e
            )
        });

    Config::new().probe("xext").unwrap_or_else(|e| {
        panic!(
            "Xext development libraries not found: {}.\nRun: nix develop",
            e
        )
    });

    Config::new().probe("xi").unwrap_or_else(|e| {
        panic!(
            "Xi development libraries not found: {}.\nRun: nix develop",
            e
        )
    });

    println!("cargo:rerun-if-changed=build.rs");
}
