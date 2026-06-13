pub mod analyzer;
pub mod ast;
pub mod catalog;
pub mod compiler;
pub mod content;
pub mod domain;
mod embedded_content;

pub use analyzer::evaluate;
pub use catalog::{all_modules, find_exercise};
pub use domain::{
    CheckResult, CompileMode, CompileReport, Difficulty, Exercise, LearningResource,
    PatternGroupMode, Rule, RuleKind, SolutionGuide, TrainingModule, ValidationReport,
};
