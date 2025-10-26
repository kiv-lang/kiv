//! Run command implementation.

use kivc::compiler::compile_source;
use kivc::config::{CompilerConfig, OptLevel};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/// Execute the run command (compile and run)
pub fn execute_run(file: &Path, opt_level: OptLevel) -> i32 {
    // Read source file
    let source = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file.display(), e);
            return 1;
        }
    };

    // Compile to LLVM IR
    let config = CompilerConfig {
        opt_level,
        stop_after: None,
        measure_time: false,
    };
    let result = compile_source(&source, file.to_str().unwrap_or("input.kiv"), &config);

    // Handle diagnostics
    if !result.diagnostics.diagnostics().is_empty() {
        for diag in result.diagnostics.diagnostics() {
            eprintln!("{}", diag);
        }
    }

    // Check for errors
    if result.has_errors() {
        eprintln!("\nCompilation failed. Cannot run.");
        return 1;
    }

    // Get LLVM IR
    let llvm_ir = match &result.llvm_ir {
        Some(ir) => ir,
        None => {
            eprintln!("No LLVM IR generated.");
            return 1;
        }
    };

    // Write temporary LLVM IR file
    let temp_ll = file.with_extension("ll");
    if let Err(e) = fs::write(&temp_ll, llvm_ir) {
        eprintln!("Error writing temporary file: {}", e);
        return 1;
    }

    // Compile LLVM IR to object file using llc
    let temp_obj = file.with_extension("o");
    let llc_status = Command::new("llc")
        .arg("-filetype=obj")
        .arg(&temp_ll)
        .arg("-o")
        .arg(&temp_obj)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match llc_status {
        Ok(status) if !status.success() => {
            eprintln!("Failed to compile LLVM IR to object file.");
            cleanup_files(&[&temp_ll, &temp_obj]);
            return 1;
        }
        Err(e) => {
            eprintln!("Error running llc: {}", e);
            cleanup_files(&[&temp_ll, &temp_obj]);
            return 1;
        }
        _ => {}
    }

    // Link with clang
    let executable = file.with_extension(if cfg!(windows) { "exe" } else { "" });
    let link_status = Command::new("clang")
        .arg(&temp_obj)
        .arg("-o")
        .arg(&executable)
        .arg("--target=x86_64-pc-windows-msvc")
        .arg("-lws2_32")
        .arg("-ladvapi32")
        .arg("-luserenv")
        .arg("-lntdll")
        .arg("-lkernel32")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match link_status {
        Ok(status) if !status.success() => {
            eprintln!("Failed to link executable.");
            cleanup_files(&[&temp_ll, &temp_obj]);
            return 1;
        }
        Err(e) => {
            eprintln!("Error running clang: {}", e);
            cleanup_files(&[&temp_ll, &temp_obj]);
            return 1;
        }
        _ => {}
    }

    // Run the executable
    println!("\nRunning {}...\n", executable.display());
    let run_status = Command::new(&executable)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    let exit_code = match run_status {
        Ok(status) => status.code().unwrap_or(1),
        Err(e) => {
            eprintln!("Error running executable: {}", e);
            1
        }
    };

    // Cleanup
    cleanup_files(&[&temp_ll, &temp_obj, &executable]);

    exit_code
}

/// Clean up temporary files
fn cleanup_files(files: &[&Path]) {
    for file in files {
        let _ = fs::remove_file(file);
    }
}
