use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_ref() {
        "windows" => {
            if target_arch == "x86_64" {
                println!("cargo:rustc-cfg=unwind");
            }
        },
        "linux" => {
            println!("cargo:rustc-cfg=unwind");
            
            // Add library search path for aarch64
            if target_arch == "aarch64" {
                println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu");
                // Link to the correct library names for aarch64
                println!("cargo:rustc-link-lib=unwind");
                println!("cargo:rustc-link-lib=unwind-ptrace");
                println!("cargo:rustc-link-lib=unwind-aarch64");
            }
        },
        _ => {}
    }
}
