use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-link-search=native=../native/build/Debug");
    println!("cargo:rustc-link-lib=static=lucus-kv-native");
    println!("cargo:rerun-if-changed=../native/build/Debug/liblucus-kv-native.a");

    tonic_prost_build::compile_protos("../proto/health.proto")?;
    println!("cargo:rerun-if-changed=../proto");

    Ok(())
}
