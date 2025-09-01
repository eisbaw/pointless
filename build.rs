use std::process::Command;

fn main() {
    // Use pkg-config to find X11 libraries - this is the most reliable approach in Nix
    if let Ok(output) = Command::new("pkg-config")
        .args(&["--cflags-only-I", "x11"])
        .output()
    {
        if output.status.success() {
            let includes = String::from_utf8_lossy(&output.stdout);
            for include in includes.split_whitespace() {
                if let Some(path) = include.strip_prefix("-I") {
                    println!("cargo:rustc-link-search=native={}", path.replace("/include", "/lib"));
                }
            }
        }
    }

    if let Ok(output) = Command::new("pkg-config")
        .args(&["--libs", "x11"])
        .output()
    {
        if output.status.success() {
            let libs = String::from_utf8_lossy(&output.stdout);
            for lib in libs.split_whitespace() {
                if let Some(path) = lib.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={}", path);
                } else if let Some(lib_name) = lib.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib={}", lib_name);
                }
            }
        }
    }

    // Also add common X11 libraries
    println!("cargo:rustc-link-lib=Xext");
    println!("cargo:rustc-link-lib=Xi");
}
