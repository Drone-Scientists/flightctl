fn main() {
    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("stdc++")
        .flag_if_supported("-std=c++17")
        .include("src/include")
        .file("src/shim.cc")
        .file("src/helper.cc")
        .compile("mavsdk");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/shim.cc");
    println!("cargo:rerun-if-changed=include/shim.h");
    println!("cargo:rustc-link-lib=mavsdk")
}
