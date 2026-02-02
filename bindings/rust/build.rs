use std::env;
use std::path::PathBuf;

fn main() {
    // Get the build output directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Link to the C++ wrapper library
    // By default, we'll look in ../../build for the shared library
    let build_dir = PathBuf::from(&manifest_dir)
        .join("../..")
        .join("build");

    // Also check common install locations
    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");

    // Link the wrapper library
    println!("cargo:rustc-link-lib=dylib=daffrustwrapper");

    // Also link the main DAFF library
    println!("cargo:rustc-link-lib=dylib=DAFF");

    // Link the C++ standard library
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    // Rerun if the wrapper changes
    println!("cargo:rerun-if-changed=daff_rust_wrapper.h");
    println!("cargo:rerun-if-changed=daff_rust_wrapper.cpp");

    // Set rpath for finding shared libraries at runtime
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", build_dir.display());
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", build_dir.display());
    }
}
