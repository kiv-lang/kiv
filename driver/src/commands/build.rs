//! `kiv build` command implementation.

use crate::project::Project;
use kivc::{CompilerConfig, OptLevel, compile_source};
use std::env;
use std::fs;
use std::process::Command;

/// Builds the Kiv project
pub fn build(release: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let project = Project::find_root(&current_dir)?;

    println!(
        "   Compiling {} v{} ({})",
        project.manifest.package.name,
        project.manifest.package.version,
        project.root.display()
    );

    // Ensure build directories exist
    project.ensure_build_dirs()?;

    // Read source file
    let main_file = project.main_file();
    if !main_file.exists() {
        return Err(format!("Source file not found: {}", main_file.display()).into());
    }

    let source = fs::read_to_string(&main_file)?;

    // Configure compiler
    let config = CompilerConfig {
        opt_level: if release {
            OptLevel::Aggressive
        } else {
            OptLevel::None
        },
        ..Default::default()
    };

    // Compile to LLVM IR
    let output = compile_source(&source, &main_file.to_string_lossy(), &config);

    if output.diagnostics.has_errors() {
        // Print diagnostics
        for error in output.diagnostics.errors() {
            eprintln!("{:?}", miette::Report::new(error.clone()));
        }
        return Err("Compilation failed".into());
    }

    match output.llvm_ir {
        Some(llvm_ir) => {
            // Write LLVM IR to file
            let ll_path = project.output_ll(release);
            fs::write(&ll_path, &llvm_ir)?;

            // Use LLVM tools to generate object file and link
            let obj_path = ll_path.with_extension("o");
            let output_path = project.output_binary(release);

            // Compile LLVM IR to object file using llc
            let llc_status = Command::new("llc")
                .arg(&ll_path)
                .arg("-filetype=obj")
                .arg("-o")
                .arg(&obj_path)
                .status();

            match llc_status {
                Ok(status) if status.success() => {
                    // Link object file with runtime library
                    let runtime_lib = if cfg!(target_os = "windows") {
                        "kivc_runtime.lib"
                    } else {
                        "libkivc_runtime.a"
                    };

                    // Get the absolute path to the runtime library
                    let runtime_lib_path = std::env::current_dir()
                        .ok()
                        .and_then(|cwd| {
                            // Try to find the library in target/debug relative to project root
                            let mut path = cwd.clone();
                            // If we're in a subdirectory, go up to find target
                            loop {
                                let candidate = path.join("target").join("debug").join(runtime_lib);
                                if candidate.exists() {
                                    return Some(candidate);
                                }
                                if !path.pop() {
                                    break;
                                }
                            }
                            None
                        })
                        .unwrap_or_else(|| {
                            // Fallback to the original relative path
                            std::path::PathBuf::from(format!("target/debug/{}", runtime_lib))
                        });

                    let link_status = Command::new("clang")
                        .arg(&obj_path)
                        .arg(&runtime_lib_path)
                        .arg("-o")
                        .arg(&output_path)
                        .arg("--target=x86_64-pc-windows-msvc")
                        .arg("-lws2_32")
                        .arg("-ladvapi32")
                        .arg("-luserenv")
                        .arg("-lntdll")
                        .arg("-lkernel32")
                        .status();

                    match link_status {
                        Ok(status) if status.success() => {
                            println!(
                                "    Finished {} [{}] target(s) in ...",
                                if release { "release" } else { "debug" },
                                if release { "optimized" } else { "unoptimized" }
                            );
                            println!();
                            println!("Binary: {}", output_path.display());
                            Ok(())
                        }
                        Ok(status) => Err(format!("Linking failed with status: {}", status).into()),
                        Err(e) => Err(format!(
                            "Failed to run linker: {}. Make sure clang is installed.",
                            e
                        )
                        .into()),
                    }
                }
                Ok(status) => Err(format!("llc failed with status: {}", status).into()),
                Err(e) => {
                    Err(format!("Failed to run llc: {}. Make sure LLVM is installed.", e).into())
                }
            }
        }
        None => Err("Compilation did not produce LLVM IR".into()),
    }
}
