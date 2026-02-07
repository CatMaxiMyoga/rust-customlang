//! IO Operations for the Compiler

use std::{fs, io, path::PathBuf, process::Command};

const TEMP_DIR: &str = "__tmp__cs_runtime";

static BUILTINS_FILE: &[u8] = include_bytes!("../cs_runtime/Builtins.cs");
static CSPROJ_FILE: &[u8] = include_bytes!("../cs_runtime/cs_runtime.csproj");
static TYPES_FILE: &[u8] = include_bytes!("../cs_runtime/Types.cs");

#[rustfmt::skip]
const DOTNET_RID: &str = {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))] { "linux-x64" }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))] { "linux-arm64" }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))] { "win-x64" }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))] { "win-arm64" }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))] { "osx-x64" }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))] { "osx-arm64" }
};

fn get_cwd() -> io::Result<PathBuf> {
    std::env::current_dir()
}

pub fn copy_runtime() {
    let cwd: PathBuf = get_cwd().expect("Failed to get current working directory");
    let dest: PathBuf = cwd.join(TEMP_DIR);
    let mut target: PathBuf;

    fs::create_dir_all(&dest).expect("Failed to create runtime destination directory");

    target = dest.join("Builtins.cs");
    if !target.exists() {
        fs::write(&target, BUILTINS_FILE).expect("Failed to write runtime file");
    }

    target = dest.join("Types.cs");
    if !target.exists() {
        fs::write(&target, TYPES_FILE).expect("Failed to write runtime file");
    }

    target = dest.join("cs_runtime.csproj");
    if !target.exists() {
        fs::write(&target, CSPROJ_FILE).expect("Failed to write runtime file");
    }
}

pub fn write_file(cs_code: &str) {
    let cwd: PathBuf = get_cwd().expect("Failed to get current working directory");
    let runtime_dir: PathBuf = cwd.join(TEMP_DIR);
    let file_path: PathBuf = runtime_dir.join("Program.cs");

    fs::write(file_path, cs_code).expect("Failed to write C# code to file");
}

pub fn call_compiler() {
    let cwd: PathBuf = get_cwd().expect("Failed to get current working directory");
    let runtime_dir: PathBuf = cwd.join(TEMP_DIR);

    #[rustfmt::skip]
    let status = Command::new("dotnet")
        .args([
            "publish",
            "-c", "Release",
            "-r", DOTNET_RID,
            "--self-contained", "true",
            "/p:PublishSingleFile=true",
            "/p:DebugType=None",
            "/p:DebugSymbols=false",
            "-o", ".."
        ])
        .current_dir(&runtime_dir)
        .status()
        .expect("Failed to execute dotnet publish command");

    if !status.success() {
        eprintln!("Dotnet publish command failed with status: {status}");
        std::process::exit(1);
    }
}

pub fn cleanup_temp_files() {
    let cwd: PathBuf = get_cwd().expect("Failed to get current working directory");
    let runtime_dir: PathBuf = cwd.join(TEMP_DIR);

    fs::remove_dir_all(&runtime_dir).expect("Failed to remove temporary runtime directory");
}

pub fn move_executable(output_file: &str) {
    let cwd: PathBuf = get_cwd().expect("Failed to get current working directory");
    #[rustfmt::skip]
    let temp_exe: PathBuf = cwd.join({
        #[cfg(target_os = "windows")] { "__tmp__customlang.exe" }
        #[cfg(not(target_os = "windows"))] { "__tmp__customlang" }
    });
    let dest_exe: PathBuf = cwd.join(output_file);

    if let Some(dest_dir) = dest_exe.parent() {
        if fs::create_dir_all(dest_dir).is_err() {
            eprintln!("Failed to create output directory, skipping...");
        }
    } else {
        eprintln!("Failed to get output directory, skipping...");
    }

    fs::rename(temp_exe, dest_exe).expect("Failed to move executable to output file");
}
