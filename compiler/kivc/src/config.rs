//! Compiler configuration and settings.

use std::fmt;

/// Optimization level for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptLevel {
    /// No optimizations (-O0)
    #[default]
    None,
    /// Basic optimizations (-O1)
    Less,
    /// Standard optimizations (-O2)
    Default,
    /// Aggressive optimizations (-O3)
    Aggressive,
}

impl OptLevel {
    pub fn from_u8(level: u8) -> Self {
        match level {
            0 => OptLevel::None,
            1 => OptLevel::Less,
            2 => OptLevel::Default,
            3 => OptLevel::Aggressive,
            _ => OptLevel::Default,
        }
    }
}

impl fmt::Display for OptLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptLevel::None => write!(f, "O0"),
            OptLevel::Less => write!(f, "O1"),
            OptLevel::Default => write!(f, "O2"),
            OptLevel::Aggressive => write!(f, "O3"),
        }
    }
}

/// Stage at which to stop compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopAfter {
    /// Stop after parsing (AST generation)
    Parse,
    /// Stop after HIR generation
    Hir,
    /// Stop after type checking
    TypeCheck,
    /// Stop after MIR generation
    Mir,
}

impl fmt::Display for StopAfter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopAfter::Parse => write!(f, "parse"),
            StopAfter::Hir => write!(f, "hir"),
            StopAfter::TypeCheck => write!(f, "typecheck"),
            StopAfter::Mir => write!(f, "mir"),
        }
    }
}

/// Compiler configuration
#[derive(Debug, Clone, Default)]
pub struct CompilerConfig {
    /// Optimization level
    #[allow(clippy::derivable_impls)]
    pub opt_level: OptLevel,

    /// Stop compilation after a specific stage
    pub stop_after: Option<StopAfter>,

    /// Measure compilation time for each stage
    pub measure_time: bool,
}

impl CompilerConfig {
    pub fn new(opt_level: OptLevel) -> Self {
        Self {
            opt_level,
            stop_after: None,
            measure_time: false,
        }
    }

    pub fn with_stop_after(mut self, stop_after: StopAfter) -> Self {
        self.stop_after = Some(stop_after);
        self
    }

    pub fn with_time_measurement(mut self, measure: bool) -> Self {
        self.measure_time = measure;
        self
    }
}
