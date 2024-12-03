fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=native=C:\\SDL2\\lib\\x64");
        println!("cargo:rustc-link-lib=static=SDL2");
    }
}
