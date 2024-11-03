fn main() {
    println!("cargo:rustc-link-search=native=c_api");   // Library path

    println!("cargo:rustc-link-lib=static=api");        // If using a static library
    // println!("cargo:rustc-link-lib=dylib=api");      // If using a dynamic library
}
