//! The compiler module for turning transpiled code into executable binaries.

mod io;

/// The compiler for the transpiled code
pub struct Compiler;

impl Compiler {
    /// Compiles the given C# code into an executable binary.
    ///
    /// Arguments:
    /// - `cs_code`: The C# code to compile as a string slice.
    pub fn compile(cs_code: &str, output_file: Option<&str>) {
        io::copy_runtime();
        io::write_file(cs_code);
        io::call_compiler();
        io::cleanup_temp_files();

        #[rustfmt::skip]
        io::move_executable(output_file.unwrap_or(
            {
                #[cfg(target_os = "windows")] { "output.exe" }
                #[cfg(not(target_os = "windows"))] { "output" }
            }
        ));
    }
}
