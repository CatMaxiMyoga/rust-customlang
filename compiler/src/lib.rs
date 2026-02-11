//! The compiler module for turning transpiled code into executable binaries.

mod io;

/// The compiler for the transpiled code
pub struct Compiler;

impl Compiler {
    /// Compiles the given C# code into an executable binary.
    ///
    /// Arguments:
    /// - `cs_code`: The C# code to compile as a string slice.
    pub fn compile(cs_code: &str, output_file: Option<String>) {
        io::copy_runtime();
        io::write_file(cs_code);
        if !io::call_compiler() {
            #[cfg(not(debug_assertions))]
            {
                println!();
                eprintln!("Dotnet publish command failed, cleaning up temporary files...");
                io::cleanup_temp_files();
            }
            std::process::exit(1);
        }
        io::cleanup_temp_files();

        #[rustfmt::skip]
        io::move_executable(&output_file.unwrap_or_else( || {
            #[cfg(target_os = "windows")] { "output.exe".to_string() }
            #[cfg(not(target_os = "windows"))] { "output".to_string() }
        }));
    }
}
