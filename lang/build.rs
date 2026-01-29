//! Build script to copy the `cs_runtime/` directory to the output directory.

use std::{
    env,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

fn main() {
    let manifest_dir: String = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let src: PathBuf = Path::new(&manifest_dir).join("../compiler/cs_runtime");
    let profile: String = env::var("PROFILE").expect("PROFILE not set");
    let out_dir: PathBuf = Path::new(&manifest_dir)
        .join("../target")
        .join(&profile)
        .join("cs_runtime");

    copy_dir(&src, &out_dir).expect("Failed to copy cs_runtime/ directory to executable");

    println!("cargo:rerun-if-changed={}", src.display());
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest).expect("Failed to create destination directory");
    }

    for entry in fs::read_dir(src)? {
        let entry: DirEntry = entry.expect("Failed to get directory entry");

        if entry.file_type()?.is_dir() {
            continue;
        }

        let dest_path: PathBuf = dest.join(entry.file_name());

        fs::copy(entry.path(), &dest_path)?;
    }

    Ok(())
}
