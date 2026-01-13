use std::{env, fs, path::PathBuf};

use fs_extra::dir::{CopyOptions, copy};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let assets_dir = manifest_dir.join("assets");

    println!("cargo:rerun-if-changed={}", assets_dir.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));
    let dest_dir = out_dir.join("assets");

    if dest_dir.exists() {
        fs::remove_dir_all(&dest_dir).expect("failed to clean previous assets");
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;

    copy(&assets_dir, &out_dir, &options).expect("failed to copy assets");
}
