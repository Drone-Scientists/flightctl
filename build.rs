fn main() {
    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .file("src/shim.cc")
        .file("src/helper.cc")
        .flag_if_supported("-std=c++17")
        .compile("mavsdk");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/shim.cc");
    println!("cargo:rerun-if-changed=include/shim.h");
    println!("cargo:rustc-link-lib=mavsdk")
}
