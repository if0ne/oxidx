fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "pix")]
    copy_pix_files()?;

    Ok(())
}

#[cfg(feature = "pix")]
fn copy_pix_files() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::fs::{self};
    use std::path::PathBuf;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let target_dir = out_dir
        .parent()
        .and_then(|dir| dir.parent())
        .and_then(|dir| dir.parent())
        .expect("failed to get target dir");

    let link_path = PathBuf::from("./external/pix/bin/win64");
    let dll_file = "WinPixEventRuntime.dll";
    let lib_file = "WinPixEventRuntime.lib";

    fs::copy(link_path.join(dll_file), target_dir.join(dll_file))?;
    fs::copy(link_path.join(lib_file), target_dir.join(lib_file))?;

    println!("cargo:rustc-link-search={}", target_dir.display());
    println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime");

    Ok(())
}
