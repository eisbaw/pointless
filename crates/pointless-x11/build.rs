fn main() {
    // Find and link X11 libraries
    if let Err(e) = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("x11")
    {
        println!("cargo:warning=Could not find x11 via pkg-config: {}", e);
        // Fallback to manual linking if pkg-config fails
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xext");
    }

    // Try to find additional X11 libraries that might be needed
    let _ = pkg_config::Config::new()
        .atleast_version("1.0")
        .probe("xext");
}
