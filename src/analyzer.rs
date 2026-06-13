use regex::Regex;

use crate::ast::{AstAnalysis, AstFacts, analyze_answer};
use crate::compiler::compile_answer;
use crate::domain::{
    AstCheck, CheckResult, CompileMode, Exercise, PatternGroupMode, Rule, RuleKind,
    ValidationReport,
};

pub fn evaluate(exercise: &Exercise, answer: &str) -> ValidationReport {
    let compile = match exercise.compile_mode {
        CompileMode::Off => None,
        mode => Some(compile_answer(answer, mode)),
    };

    let ast = analyze_answer(answer, exercise.compile_mode);

    let mut checks = Vec::with_capacity(exercise.rules.len());
    for rule in &exercise.rules {
        checks.push(evaluate_rule(rule, answer, &ast));
    }

    let rules_ok = checks.iter().all(|check| check.ok);
    let compile_ok = compile.as_ref().is_none_or(|report| report.ok);

    ValidationReport {
        exercise_id: exercise.id.to_string(),
        passed: rules_ok && compile_ok,
        checks,
        compile,
    }
}

fn evaluate_rule(rule: &Rule, answer: &str, ast: &AstAnalysis) -> CheckResult {
    match &rule.kind {
        RuleKind::RequiredAst { check } => evaluate_ast_rule(rule, check, ast, false),
        RuleKind::ForbiddenAst { check } => evaluate_ast_rule(rule, check, ast, true),
        RuleKind::RequiredPattern { pattern } => match compile_regex(pattern) {
            Ok(regex) => {
                let ok = regex.is_match(answer);
                result(rule, ok, None)
            }
            Err(err) => invalid_rule(rule, err),
        },
        RuleKind::ForbiddenPattern { pattern } => match compile_regex(pattern) {
            Ok(regex) => {
                let ok = !regex.is_match(answer);
                result(rule, ok, None)
            }
            Err(err) => invalid_rule(rule, err),
        },
        RuleKind::MinPatternCount { pattern, min } => match compile_regex(pattern) {
            Ok(regex) => {
                let count = regex.find_iter(answer).count();
                let ok = count >= *min;
                result(
                    rule,
                    ok,
                    Some(format!("Encontrado: {count}; mínimo: {min}.")),
                )
            }
            Err(err) => invalid_rule(rule, err),
        },
        RuleKind::PatternGroup { mode, patterns } => {
            let mut matches = Vec::with_capacity(patterns.len());

            for pattern in patterns {
                match compile_regex(pattern) {
                    Ok(regex) => matches.push(regex.is_match(answer)),
                    Err(err) => return invalid_rule(rule, err),
                }
            }

            let ok = match mode {
                PatternGroupMode::Any => matches.iter().any(|matched| *matched),
                PatternGroupMode::All => matches.iter().all(|matched| *matched),
            };

            result(rule, ok, None)
        }
    }
}

fn evaluate_ast_rule(
    rule: &Rule,
    check: &AstCheck,
    ast: &AstAnalysis,
    forbidden: bool,
) -> CheckResult {
    let facts = match ast.facts() {
        Ok(facts) => facts,
        Err(error) => {
            return CheckResult {
                id: rule.id.to_string(),
                ok: false,
                message: "Não foi possível analisar a AST do código.".to_string(),
                detail: Some(error.to_string()),
            };
        }
    };

    let (matched, detail) = evaluate_ast_check(check, facts);
    let ok = if forbidden { !matched } else { matched };

    result(rule, ok, detail)
}

fn evaluate_ast_check(check: &AstCheck, facts: &AstFacts) -> (bool, Option<String>) {
    match check {
        AstCheck::HasLetInitializer => (facts.has_let_initializer(), None),
        AstCheck::HasLetInitializerWithInt(value) => {
            (facts.has_let_initializer_with_int(*value), None)
        }
        AstCheck::HasLetInitializerWithPath(path) => {
            (facts.has_let_initializer_with_path(path), None)
        }
        AstCheck::HasLetInitializerWithAnyPath => (facts.has_let_initializer_with_any_path(), None),
        AstCheck::HasLetInitializerWithCallPath(path) => {
            (facts.has_let_initializer_with_call_path(path), None)
        }
        AstCheck::HasLetInitializerWithCallPathWithIntArg { path, arg } => (
            facts.has_let_initializer_with_call_path_and_int_arg(path, *arg),
            None,
        ),
        AstCheck::HasLetInitializerWithDeref => (facts.has_let_initializer_with_deref(), None),
        AstCheck::HasLetInitializerWithDerefPath(path) => {
            (facts.has_let_initializer_with_deref_path(path), None)
        }
        AstCheck::HasLetInitializerWithIf => (facts.has_let_initializer_with_if(), None),
        AstCheck::HasStructPatternField { path, field } => {
            (facts.has_struct_pattern_field(path, field), None)
        }
        AstCheck::HasTuplePatternBindingCount { count } => {
            (facts.has_tuple_pattern_binding_count(*count), None)
        }
        AstCheck::HasLetMut => (facts.has_let_mut(), None),
        AstCheck::HasLetType(type_name) => (facts.has_let_type(type_name), None),
        AstCheck::HasCallPath(path) => (facts.has_call_path(path), None),
        AstCheck::HasCallPathWithIntArg { path, arg } => {
            (facts.has_call_path_with_int_arg(path, *arg), None)
        }
        AstCheck::HasMethodCall(method) => (facts.has_method_call(method), None),
        AstCheck::HasMacroCall(macro_name) => (facts.has_macro_call(macro_name), None),
        AstCheck::HasForLoop => (facts.has_for_loop(), None),
        AstCheck::HasForLoopByReference => (facts.has_for_loop_by_reference(), None),
        AstCheck::HasWhileLoop => (facts.has_while_loop(), None),
        AstCheck::HasWhileLetSome => (facts.has_while_let_some(), None),
        AstCheck::HasLoop => (facts.has_loop(), None),
        AstCheck::HasIf => (facts.has_if(), None),
        AstCheck::HasIfLetSome => (facts.has_if_let_some(), None),
        AstCheck::HasMatch => (facts.has_match(), None),
        AstCheck::HasFunction => (facts.has_function(), None),
        AstCheck::HasFunctionParamCount { min } => (facts.has_function_param_count(*min), None),
        AstCheck::HasFunctionReturnType => (facts.has_function_return_type(), None),
        AstCheck::HasArrayToVec => (facts.has_array_to_vec(), None),
        AstCheck::HasBinaryAdd => (facts.has_binary_add(), None),
        AstCheck::MinMethodCallCount { method, min } => {
            let count = facts.method_call_count(method);
            (
                count >= *min,
                Some(format!("Encontrado: {count}; mínimo: {min}.")),
            )
        }
    }
}

fn compile_regex(pattern: &str) -> Result<Regex, regex::Error> {
    Regex::new(pattern)
}

fn result(rule: &Rule, ok: bool, detail: Option<String>) -> CheckResult {
    CheckResult {
        id: rule.id.to_string(),
        ok,
        message: if ok { &rule.success } else { &rule.failure }.to_string(),
        detail,
    }
}

fn invalid_rule(rule: &Rule, error: regex::Error) -> CheckResult {
    CheckResult {
        id: rule.id.to_string(),
        ok: false,
        message: "Regra de validação inválida.".to_string(),
        detail: Some(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::{all_modules, evaluate, find_exercise};

    #[test]
    fn all_catalog_starters_pass_their_own_rules() {
        for module in all_modules() {
            for exercise in module.exercises {
                let report = evaluate(&exercise, &exercise.starter);
                assert!(
                    report.passed,
                    "starter should pass for {}: {report:#?}",
                    exercise.id
                );
            }
        }
    }

    #[test]
    fn accepts_equivalent_vec_inferred_push_answer() {
        let exercise = find_exercise("vec-inferido-push").expect("exercise should exist");
        let answer = "\
let mut xs = Vec::new();
xs.push(42);
xs.push(7);
";

        let report = evaluate(&exercise, answer);

        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn accepts_structurally_valid_answer_with_unusual_spacing() {
        let exercise = find_exercise("vec-inferido-push").expect("exercise should exist");
        let answer = "\
let mut xs =
    Vec
        ::new();
xs.push(42);
xs
    .push(7);
";

        let report = evaluate(&exercise, answer);

        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn rejects_comment_only_answer_that_mentions_expected_code() {
        let exercise = find_exercise("vec-inferido-push").expect("exercise should exist");
        let answer = "\
// let mut xs = Vec::new();
// xs.push(42);
// xs.push(7);
";

        let report = evaluate(&exercise, answer);

        assert!(!report.passed);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.id == "vec-new" && !check.ok)
        );
    }

    #[test]
    fn rejects_forbidden_vec_macro_on_inferred_push_exercise() {
        let exercise = find_exercise("vec-inferido-push").expect("exercise should exist");
        let answer = "let mut xs = vec![42, 7];";

        let report = evaluate(&exercise, answer);

        assert!(!report.passed);
        assert!(
            report
                .checks
                .iter()
                .any(|check| { check.id == "sem-vec-macro" && !check.ok })
        );
    }

    #[test]
    fn rejects_code_that_matches_patterns_but_does_not_compile() {
        let exercise = find_exercise("vec-inferido-push").expect("exercise should exist");
        let answer = "\
let mut xs = Vec::new();
xs.push();
xs.push(7);
";

        let report = evaluate(&exercise, answer);

        assert!(!report.passed);
        assert!(report.compile.is_some_and(|compile| !compile.ok));
    }

    #[test]
    fn accepts_arbitrary_binding_name_when_name_is_not_the_lesson() {
        let exercise = find_exercise("let-imutavel").expect("exercise should exist");
        let report = evaluate(&exercise, "let qualquer_nome = 42;");

        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn rejects_wildcard_when_exercise_requires_a_binding() {
        let exercise = find_exercise("let-imutavel").expect("exercise should exist");
        let report = evaluate(&exercise, "let _ = 42;");

        assert!(!report.passed);
        assert!(
            report
                .checks
                .iter()
                .any(|check| check.id == "let" && !check.ok)
        );
    }

    #[test]
    fn accepts_if_let_with_arbitrary_binding_name() {
        let exercise = find_exercise("if-let-option").expect("exercise should exist");
        let answer = "\
let valor = Some(3);
let mut dobro = 0;
if let Some(outro_nome) = valor {
    dobro = outro_nome * 2;
}
";

        let report = evaluate(&exercise, answer);

        assert!(report.passed, "{report:#?}");
    }

    #[test]
    fn accepts_tuple_destructure_with_arbitrary_binding_names() {
        let exercise = find_exercise("tuple-destructure").expect("exercise should exist");
        let report = evaluate(
            &exercise,
            "let ponto = (10, 20);\nlet (coluna, linha) = ponto;",
        );

        assert!(report.passed, "{report:#?}");
    }
}
