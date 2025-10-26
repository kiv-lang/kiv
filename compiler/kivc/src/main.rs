//! # Kiv Compiler CLI
//!
//! This is the command-line interface for the Kiv compiler (kivc).

mod commands;

use clap::{Parser, Subcommand};
use commands::{execute_build, execute_check, execute_run};
use kivc::config::OptLevel;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(name = "kivc")]
#[command(about = "The Kiv Programming Language Compiler", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Kiv source file
    Build {
        /// Input file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output file (optional)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Optimization level (0-2)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,

        /// Emit LLVM IR (.ll file)
        #[arg(long)]
        emit_llvm: bool,
    },

    /// Check a Kiv source file without generating code
    Check {
        /// Input file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Optimization level (0-2)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,
    },

    /// Compile and run a Kiv source file
    Run {
        /// Input file to compile and run
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Optimization level (0-2)
        #[arg(short = 'O', long, default_value = "0")]
        opt_level: u8,
    },
}

fn main() {
    let cli = Cli::parse();
    let exit_code = match cli.command {
        Commands::Build {
            input,
            output,
            opt_level,
            emit_llvm,
        } => {
            let opt = parse_opt_level(opt_level);
            execute_build(&input, opt, output.as_deref(), emit_llvm, None)
        }

        Commands::Check { input, opt_level } => {
            let opt = parse_opt_level(opt_level);
            execute_check(&input, opt)
        }

        Commands::Run { input, opt_level } => {
            let opt = parse_opt_level(opt_level);
            execute_run(&input, opt)
        }
    };

    process::exit(exit_code);
}

/// Parse optimization level from u8
fn parse_opt_level(level: u8) -> OptLevel {
    match level {
        0 => OptLevel::None,
        1 => OptLevel::Less,
        _ => OptLevel::Aggressive,
    }
}
