fn main() {
    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("stdc++")
        .flag_if_supported("-std=c++17")
        .include("src/include")
        .file("src/helper.cc")
        .compile("mavsdk");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/helper.cc");
    println!("cargo:rerun-if-changed=include/helper.h");
    println!("cargo:rustc-link-lib=mavsdk")
}
