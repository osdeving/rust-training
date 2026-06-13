#[derive(Debug, Clone)]
pub struct TrainingModule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: String,
    pub module_id: String,
    pub title: String,
    pub prompt: String,
    pub starter: String,
    pub scaffold: String,
    pub difficulty: Difficulty,
    pub guide: SolutionGuide,
    pub hints: Vec<String>,
    pub rules: Vec<Rule>,
    pub compile_mode: CompileMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Beginner => "Iniciante",
            Difficulty::Intermediate => "Intermediario",
            Difficulty::Advanced => "Avancado",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SolutionGuide {
    pub summary: String,
    pub solution: String,
    pub concepts: Vec<String>,
    pub pitfalls: Vec<String>,
    pub docs: Vec<LearningResource>,
}

#[derive(Debug, Clone)]
pub struct LearningResource {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileMode {
    Off,
    SnippetAsMain,
    FullProgram,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub success: String,
    pub failure: String,
    pub kind: RuleKind,
}

#[derive(Debug, Clone)]
pub enum RuleKind {
    RequiredAst {
        check: AstCheck,
    },
    ForbiddenAst {
        check: AstCheck,
    },
    RequiredPattern {
        pattern: String,
    },
    ForbiddenPattern {
        pattern: String,
    },
    MinPatternCount {
        pattern: String,
        min: usize,
    },
    PatternGroup {
        mode: PatternGroupMode,
        patterns: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub enum AstCheck {
    HasLetMut,
    HasLetType(String),
    HasCallPath(String),
    HasCallPathWithIntArg { path: String, arg: u128 },
    HasMethodCall(String),
    HasMacroCall(String),
    HasForLoop,
    HasForLoopByReference,
    HasWhileLoop,
    HasLoop,
    HasIf,
    HasMatch,
    HasFunction,
    HasFunctionParamCount { min: usize },
    HasFunctionReturnType,
    HasArrayToVec,
    HasBinaryAdd,
    MinMethodCallCount { method: String, min: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternGroupMode {
    Any,
    All,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: String,
    pub ok: bool,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompileReport {
    pub ok: bool,
    pub timed_out: bool,
    pub stderr: String,
}

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub exercise_id: String,
    pub passed: bool,
    pub checks: Vec<CheckResult>,
    pub compile: Option<CompileReport>,
}

impl Rule {
    pub fn required_ast(
        id: impl Into<String>,
        check: AstCheck,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::RequiredAst { check },
        }
    }

    pub fn forbidden_ast(
        id: impl Into<String>,
        check: AstCheck,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::ForbiddenAst { check },
        }
    }

    pub fn required_pattern(
        id: impl Into<String>,
        pattern: impl Into<String>,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::RequiredPattern {
                pattern: pattern.into(),
            },
        }
    }

    pub fn forbidden_pattern(
        id: impl Into<String>,
        pattern: impl Into<String>,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::ForbiddenPattern {
                pattern: pattern.into(),
            },
        }
    }

    pub fn min_pattern_count(
        id: impl Into<String>,
        pattern: impl Into<String>,
        min: usize,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::MinPatternCount {
                pattern: pattern.into(),
                min,
            },
        }
    }

    pub fn any_pattern(
        id: impl Into<String>,
        patterns: Vec<String>,
        success: impl Into<String>,
        failure: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            success: success.into(),
            failure: failure.into(),
            kind: RuleKind::PatternGroup {
                mode: PatternGroupMode::Any,
                patterns,
            },
        }
    }
}
