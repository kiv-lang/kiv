//! Common test utilities for integration tests.

use kivc::{CompilerConfig, OptLevel};

pub fn compile_with_config(source: &str, opt_level: OptLevel) -> kivc::CompilerOutput {
    let config = CompilerConfig::new(opt_level);
    kivc::compile_source(source, "test.kiv", &config)
}
