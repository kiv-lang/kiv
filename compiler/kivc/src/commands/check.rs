//! Check command implementation.

use kivc::compiler::compile_source;
use kivc::config::{CompilerConfig, OptLevel, StopAfter};
use std::fs;
use std::path::Path;

/// Execute the check command (type-check without code generation)
pub fn execute_check(file: &Path, opt_level: OptLevel) -> i32 {
    // Read source file
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file.display(), e);
            return 1;
        }
    };

    // Configure compiler to stop after type checking
    let config = CompilerConfig {
        opt_level,
        stop_after: Some(StopAfter::TypeCheck),
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
        eprintln!("\nCheck failed with errors.");
        return 1;
    }

    println!("Check completed successfully. No errors found.");
    0
}
