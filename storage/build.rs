use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let profile = match std::env::var("PROFILE")?.as_str() {
        "debug" => "Debug",
        "release" => "Release",
        profile => return Err(format!("Unknown profile: {profile}").into()),
    };

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../native/build")
        .join(profile);
    let lib_path = dir.join("liblucus-kv-native.a");

    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=lucus-kv-native");
    println!("cargo:rerun-if-changed={}", lib_path.display());
    println!("cargo:rerun-if-env-changed=PROFILE");

    Ok(())
}
