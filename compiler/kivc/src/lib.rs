//! # Kiv Compiler Core
//!
//! This crate provides the core compilation pipeline for the Kiv programming language.
//! It integrates all compiler stages from lexing to code generation.

pub mod compiler;
pub mod config;
pub mod session;

// Re-exports for convenience
pub use compiler::{
    CompilerOutput, compile_source, compile_to_ast, compile_to_hir, compile_to_mir,
};
pub use config::{CompilerConfig, OptLevel, StopAfter};
pub use session::CompilerSession;
