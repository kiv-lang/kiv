//! Build command implementation.

use kivc::compiler::compile_source;
use kivc::config::{CompilerConfig, OptLevel, StopAfter};
use std::fs;
use std::path::Path;

/// Execute the build command
pub fn execute_build(
    file: &Path,
    opt_level: OptLevel,
    output: Option<&Path>,
    emit_llvm: bool,
    stop_after: Option<StopAfter>,
) -> i32 {
    // Read source file
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file.display(), e);
            return 1;
        }
    };

    // Configure compiler
    let config = CompilerConfig {
        opt_level,
        stop_after,
        measure_time: false,
    };

    // Compile
    let result = compile_source(&source, file.to_str().unwrap_or("input.kiv"), &config);

    // Handle diagnostics
    if !result.diagnostics.diagnostics().is_empty() {
        for diag in result.diagnostics.diagnostics() {
            eprintln!("{}", diag);
        }
    }

    // Check for errors
    if result.has_errors() {
        eprintln!("\nCompilation failed with errors.");
        return 1;
    }

    // Handle output
    if let Some(llvm_ir) = &result.llvm_ir {
        // Determine output file
        let output_file = output
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| file.with_extension("ll"));

        // Write LLVM IR to file
        if let Err(e) = fs::write(&output_file, llvm_ir) {
            eprintln!("Error writing output file: {}", e);
            return 1;
        }

        println!("Successfully compiled to {}", output_file.display());

        // Optionally emit separate .ll file
        if emit_llvm {
            let ll_file = file.with_extension("ll");
            if ll_file != output_file {
                if let Err(e) = fs::write(&ll_file, llvm_ir) {
                    eprintln!("Error writing LLVM IR file: {}", e);
                    return 1;
                }
                println!("LLVM IR written to {}", ll_file.display());
            }
        }
    } else {
        println!("Compilation stopped at intermediate stage.");
    }

    0
}
