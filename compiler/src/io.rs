use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use glob::glob;

pub fn cleanup(clean_up: bool) {
    if !clean_up {
        return;
    }

    let out_dir = Path::new("out");

    if let Ok(entries) = fs::read_dir(out_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let _ = fs::remove_file(&path);
            } else if path.is_dir() {
                let _ = fs::remove_dir_all(&path);
            }
        }
    }
}

pub fn copy_runtime_files() -> Result<(), String> {
    #[cfg(not(debug_assertions))]
    let exe_dir: PathBuf = std::env::current_exe()
        .map_err(|e| format!("Failed to get current exe path: {e}"))?
        .parent()
        .ok_or("Failed to get parent directory of current exe")?
        .to_path_buf();
    #[cfg(not(debug_assertions))]
    let paths: [PathBuf; 2] = [exe_dir.join("c_runtime/*.c"), exe_dir.join("c_runtime/*.h")];
    #[cfg(not(debug_assertions))]
    let bindings: [String; 2] = [
        paths[0].to_string_lossy().replace('\\', "/"),
        paths[1].to_string_lossy().replace('\\', "/"),
    ];
    #[cfg(not(debug_assertions))]
    let patterns: [&str; 2] = [&bindings[0], &bindings[1]];

    #[cfg(debug_assertions)]
    let patterns: [&str; 2] = ["compiler/c_runtime/*.c", "compiler/c_runtime/*.h"];

    for pattern in patterns {
        for entry in glob(pattern).map_err(|_| "Invalid glob pattern")? {
            let path: PathBuf = entry.map_err(|_| "Failed to read a path")?;
            let file_name: &std::ffi::OsStr = path.file_name().ok_or("Invalid file name")?;

            let dest: PathBuf = PathBuf::from("out").join(file_name);

            fs::copy(&path, &dest).map_err(|_| format!("Failed to copy {}", path.display()))?;
        }
    }

    Ok(())
}

pub fn gcc_compile(out_arg: [&str; 2], gcc_args: &[String], out_file: &str) -> Result<(), String> {
    let mut cmd: Command = Command::new("gcc");

    cmd.current_dir("out");

    for entry in fs::read_dir("out").map_err(|_| "Failed to read out/ dir")? {
        let path: PathBuf = entry.map_err(|_| "Failed to read a path")?.path();
        let file_name: &std::ffi::OsStr = path.file_name().ok_or("Invalid file name")?;

        if path.extension().is_some_and(|ext| ext == "c") {
            cmd.arg(file_name);
        }
    }

    cmd.args(out_arg);
    cmd.args(gcc_args);

    if !cmd.status().map_err(|_| "Failed to run gcc")?.success() {
        return Err(String::from("GCC compilation failed"));
    }

    if out_arg.is_empty() {
        println!("Output is in out/");
    } else {
        println!("Output binary is at out/{out_file}");
    }

    Ok(())
}

pub fn cleanup_temp_files() -> Result<(), String> {
    let out_dir: &Path = Path::new("out");

    for entry in fs::read_dir(out_dir).map_err(|_| "Failed to read out/ dir")? {
        let path: PathBuf = entry.map_err(|_| "Invalid entry")?.path();

        if let Some(ext) = path.extension()
            && (ext == "c" || ext == "h")
        {
            fs::remove_file(&path).map_err(|_| format!("Failed to remove {}", path.display()))?;
        }
    }

    Ok(())
}
