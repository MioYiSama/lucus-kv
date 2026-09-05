fn main() {
    println!("cargo:rustc-link-search=native=../native/build/Debug");
    println!("cargo:rustc-link-lib=static=lucus-kv-native");
}
