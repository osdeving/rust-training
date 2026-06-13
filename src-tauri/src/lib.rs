use rust_training::{all_modules, evaluate, find_exercise, rustc_status};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ModuleDto {
    id: String,
    title: String,
    description: String,
    exercises: Vec<ExerciseSummaryDto>,
}

#[derive(Debug, Serialize)]
struct ExerciseSummaryDto {
    id: String,
    title: String,
    difficulty: String,
}

#[derive(Debug, Serialize)]
struct ExerciseDto {
    id: String,
    module_id: String,
    title: String,
    prompt: String,
    scaffold: String,
    difficulty: String,
    rubric: Vec<RubricItemDto>,
    guide: SolutionGuideDto,
    hint_count: usize,
}

#[derive(Debug, Serialize)]
struct RubricItemDto {
    id: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct SolutionGuideDto {
    summary: String,
    solution: String,
    concepts: Vec<String>,
    pitfalls: Vec<String>,
    docs: Vec<LearningResourceDto>,
}

#[derive(Debug, Serialize)]
struct LearningResourceDto {
    title: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct HintDto {
    index: usize,
    total: usize,
    text: String,
}

#[derive(Debug, Serialize)]
struct ValidationReportDto {
    exercise_id: String,
    passed: bool,
    checks: Vec<CheckResultDto>,
    compile: Option<CompileReportDto>,
}

#[derive(Debug, Serialize)]
struct CheckResultDto {
    id: String,
    ok: bool,
    message: String,
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
struct CompileReportDto {
    ok: bool,
    timed_out: bool,
    stderr: String,
}

#[derive(Debug, Serialize)]
struct ToolchainStatusDto {
    available: bool,
    command: Option<String>,
    version: Option<String>,
    message: String,
    install_url: String,
}

#[tauri::command]
fn get_catalog() -> Vec<ModuleDto> {
    all_modules()
        .into_iter()
        .map(|module| ModuleDto {
            id: module.id.to_string(),
            title: module.title.to_string(),
            description: module.description.to_string(),
            exercises: module
                .exercises
                .into_iter()
                .map(|exercise| ExerciseSummaryDto {
                    id: exercise.id.to_string(),
                    title: exercise.title.to_string(),
                    difficulty: exercise.difficulty.label().to_string(),
                })
                .collect(),
        })
        .collect()
}

#[tauri::command]
fn get_exercise(id: String) -> Result<ExerciseDto, String> {
    let exercise = find_exercise(&id).ok_or_else(|| format!("Exercicio nao encontrado: {id}"))?;

    Ok(ExerciseDto {
        id: exercise.id.to_string(),
        module_id: exercise.module_id.to_string(),
        title: exercise.title.to_string(),
        prompt: exercise.prompt.to_string(),
        scaffold: exercise.scaffold.to_string(),
        difficulty: exercise.difficulty.label().to_string(),
        rubric: exercise
            .rules
            .iter()
            .map(|rule| RubricItemDto {
                id: rule.id.to_string(),
                text: rule.failure.to_string(),
            })
            .collect(),
        guide: SolutionGuideDto {
            summary: exercise.guide.summary.to_string(),
            solution: exercise.guide.solution.to_string(),
            concepts: exercise.guide.concepts.into_iter().collect(),
            pitfalls: exercise.guide.pitfalls.into_iter().collect(),
            docs: exercise
                .guide
                .docs
                .into_iter()
                .map(|doc| LearningResourceDto {
                    title: doc.title.to_string(),
                    url: doc.url.to_string(),
                })
                .collect(),
        },
        hint_count: exercise.hints.len(),
    })
}

#[tauri::command]
fn get_hint(id: String, index: usize) -> Result<HintDto, String> {
    if index == 0 {
        return Err("A dica deve comecar em 1.".to_string());
    }

    let exercise = find_exercise(&id).ok_or_else(|| format!("Exercicio nao encontrado: {id}"))?;
    let text = exercise.hints.get(index - 1).ok_or_else(|| {
        format!(
            "Dica nao encontrada. Este exercicio tem {} dica(s).",
            exercise.hints.len()
        )
    })?;

    Ok(HintDto {
        index,
        total: exercise.hints.len(),
        text: text.to_string(),
    })
}

#[tauri::command]
fn check_answer(id: String, answer: String) -> Result<ValidationReportDto, String> {
    let exercise = find_exercise(&id).ok_or_else(|| format!("Exercicio nao encontrado: {id}"))?;
    let report = evaluate(&exercise, &answer);

    Ok(ValidationReportDto {
        exercise_id: report.exercise_id,
        passed: report.passed,
        checks: report
            .checks
            .into_iter()
            .map(|check| CheckResultDto {
                id: check.id,
                ok: check.ok,
                message: check.message,
                detail: check.detail,
            })
            .collect(),
        compile: report.compile.map(|compile| CompileReportDto {
            ok: compile.ok,
            timed_out: compile.timed_out,
            stderr: compile.stderr,
        }),
    })
}

#[tauri::command]
fn get_toolchain_status() -> ToolchainStatusDto {
    let status = rustc_status();
    ToolchainStatusDto {
        available: status.available,
        command: status.command,
        version: status.version,
        message: status.message,
        install_url: status.install_url,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_catalog,
            get_exercise,
            get_hint,
            check_answer,
            get_toolchain_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
