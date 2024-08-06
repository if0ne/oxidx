fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::{self};
    use std::path::PathBuf;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let link_path = PathBuf::from("./external/pix/bin/win64");

    let dll_file = "WinPixEventRuntime.dll";
    let lib_file = "WinPixEventRuntime.lib";
    fs::copy(link_path.join(dll_file), out_path.join(dll_file))?;
    fs::copy(link_path.join(lib_file), out_path.join(lib_file))?;

    println!("cargo:rustc-link-search={}", out_path.display());
    println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime");

    Ok(())
}
