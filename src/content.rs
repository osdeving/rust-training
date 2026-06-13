use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};
use crate::embedded_content;

#[derive(Debug)]
pub struct ContentError {
    message: String,
}

impl ContentError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

pub fn load_modules() -> Result<Vec<TrainingModule>, ContentError> {
    if let Ok(path) = std::env::var("RUST_TRAINING_CONTENT_DIR") {
        return load_modules_from_dir(PathBuf::from(path));
    }

    if let Some(dir) = content_dir()? {
        return load_modules_from_dir(dir);
    }

    load_embedded_modules()
}

fn load_modules_from_dir(dir: PathBuf) -> Result<Vec<TrainingModule>, ContentError> {
    let mut paths = fs::read_dir(&dir)
        .map_err(|error| ContentError::new(format!("nao foi possivel ler {:?}: {error}", dir)))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| ContentError::new(error.to_string()))?;

    paths.retain(|path| path.extension().is_some_and(|extension| extension == "md"));
    paths.sort();

    let mut entries = Vec::new();
    for path in paths {
        let source = fs::read_to_string(&path)
            .map_err(|error| ContentError::new(format!("falha lendo {:?}: {error}", path)))?;
        let module = parse_module(&source, &path)?;
        entries.push((path, module));
    }

    if let Err(error) = sync_catalog_summary(&dir, &entries) {
        eprintln!("Falha ao atualizar sumario do catalogo: {error}");
    }

    let modules = entries
        .into_iter()
        .map(|(_, module)| module)
        .collect::<Vec<_>>();

    Ok(modules)
}

fn load_embedded_modules() -> Result<Vec<TrainingModule>, ContentError> {
    embedded_content::MODULES
        .iter()
        .map(|(name, source)| parse_module(source, Path::new(name)))
        .collect()
}

fn sync_catalog_summary(
    modules_dir: &Path,
    entries: &[(PathBuf, TrainingModule)],
) -> Result<(), ContentError> {
    let Some(content_dir) = modules_dir.parent() else {
        return Ok(());
    };
    let path = content_dir.join("CATALOG.md");
    let expected = render_catalog_summary(entries);

    match fs::read_to_string(&path) {
        Ok(current) if current == expected => Ok(()),
        Ok(_) | Err(_) => fs::write(&path, expected)
            .map_err(|error| ContentError::new(format!("falha escrevendo {:?}: {error}", path))),
    }
}

fn render_catalog_summary(entries: &[(PathBuf, TrainingModule)]) -> String {
    let module_count = entries.len();
    let exercise_count = entries
        .iter()
        .map(|(_, module)| module.exercises.len())
        .sum::<usize>();

    let mut output = String::new();
    output.push_str("# Catalogo de exercicios\n\n");
    output.push_str("> Arquivo gerado automaticamente a partir de `content/modules/*.md`. ");
    output.push_str("Nao edite manualmente; altere os modulos e recarregue o app.\n\n");
    output.push_str(&format!(
        "Total: {} modulo(s), {} exercicio(s).\n\n",
        module_count, exercise_count
    ));
    output.push_str("## Modulos\n\n");
    output
        .push_str("| # | Modulo | Arquivo | Exercicios | Iniciante | Intermediario | Avancado |\n");
    output.push_str("|---:|---|---|---:|---:|---:|---:|\n");

    for (index, (path, module)) in entries.iter().enumerate() {
        let (beginner, intermediate, advanced) = difficulty_counts(module);
        output.push_str(&format!(
            "| {} | {} | `{}` | {} | {} | {} | {} |\n",
            index + 1,
            escape_table_cell(&module.title),
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<desconhecido>"),
            module.exercises.len(),
            beginner,
            intermediate,
            advanced
        ));
    }

    output.push_str("\n## Exercicios\n\n");
    for (path, module) in entries {
        output.push_str(&format!(
            "### {} (`{}`)\n\n",
            module.title,
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<desconhecido>")
        ));
        output.push_str(&format!("{}\n\n", module.description));
        for exercise in &module.exercises {
            output.push_str(&format!(
                "- `{}` - {} ({})\n",
                exercise.id,
                exercise.title,
                exercise.difficulty.label()
            ));
        }
        output.push('\n');
    }

    output
}

fn difficulty_counts(module: &TrainingModule) -> (usize, usize, usize) {
    module
        .exercises
        .iter()
        .fold((0, 0, 0), |mut counts, exercise| {
            match exercise.difficulty {
                Difficulty::Beginner => counts.0 += 1,
                Difficulty::Intermediate => counts.1 += 1,
                Difficulty::Advanced => counts.2 += 1,
            }
            counts
        })
}

fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn content_dir() -> Result<Option<PathBuf>, ContentError> {
    let current = std::env::current_dir()
        .map_err(|error| ContentError::new(format!("falha obtendo current_dir: {error}")))?;
    let candidates = [
        current.join("content/modules"),
        current.join("../content/modules"),
        std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|parent| parent.join("content/modules")))
            .unwrap_or_default(),
    ];

    Ok(candidates.into_iter().find(|path| path.is_dir()))
}

fn parse_module(source: &str, path: &Path) -> Result<TrainingModule, ContentError> {
    let mut module_meta = HashMap::new();
    let mut exercises = Vec::new();
    let mut current: Option<ExerciseBuilder> = None;

    let lines = source.lines().collect::<Vec<_>>();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].trim_end();

        if let Some(id) = line.strip_prefix("## ") {
            if let Some(builder) = current.take() {
                exercises.push(builder.build(path)?);
            }
            current = Some(ExerciseBuilder::new(id.trim()));
            index += 1;
            continue;
        }

        if line.starts_with("```") {
            let block_name = line.trim_start_matches("```").trim();
            let mut value = String::new();
            index += 1;
            while index < lines.len() && !lines[index].trim_start().starts_with("```") {
                value.push_str(lines[index]);
                value.push('\n');
                index += 1;
            }
            if value.ends_with('\n') {
                value.pop();
            }

            if let Some(builder) = current.as_mut() {
                builder.set_block(block_name, value);
            }
            index += 1;
            continue;
        }

        if let Some(item) = line.trim().strip_prefix("- ") {
            if let Some(builder) = current.as_mut() {
                builder.push_list_item(item.trim());
            }
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if !key.is_empty() {
                if let Some(builder) = current.as_mut() {
                    builder.set_meta(key, value);
                } else {
                    module_meta.insert(key.to_string(), value.to_string());
                }
            }
        } else if let Some(section) = line.strip_prefix("### ") {
            if let Some(builder) = current.as_mut() {
                builder.current_list = Some(section.trim().to_ascii_lowercase());
            }
        }

        index += 1;
    }

    if let Some(builder) = current.take() {
        exercises.push(builder.build(path)?);
    }

    Ok(TrainingModule {
        id: required_module_meta(&module_meta, "id", path)?,
        title: required_module_meta(&module_meta, "title", path)?,
        description: required_module_meta(&module_meta, "description", path)?,
        exercises,
    })
}

fn required_module_meta(
    meta: &HashMap<String, String>,
    key: &str,
    path: &Path,
) -> Result<String, ContentError> {
    meta.get(key)
        .cloned()
        .ok_or_else(|| ContentError::new(format!("{:?}: metadado de modulo ausente: {key}", path)))
}

#[derive(Default)]
struct ExerciseBuilder {
    id: String,
    meta: HashMap<String, String>,
    blocks: HashMap<String, String>,
    hints: Vec<String>,
    concepts: Vec<String>,
    pitfalls: Vec<String>,
    docs: Vec<LearningResource>,
    rules: Vec<Rule>,
    errors: Vec<String>,
    current_list: Option<String>,
}

impl ExerciseBuilder {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Self::default()
        }
    }

    fn set_meta(&mut self, key: &str, value: &str) {
        self.meta.insert(key.to_string(), value.to_string());
    }

    fn set_block(&mut self, key: &str, value: String) {
        self.blocks.insert(key.to_ascii_lowercase(), value);
    }

    fn push_list_item(&mut self, item: &str) {
        match self.current_list.as_deref() {
            Some("hints") => self.hints.push(item.to_string()),
            Some("concepts") => self.concepts.push(item.to_string()),
            Some("pitfalls") => self.pitfalls.push(item.to_string()),
            Some("docs") => {
                if let Some(doc) = parse_doc(item) {
                    self.docs.push(doc);
                }
            }
            Some("rules") => match parse_rule(item) {
                Ok(rule) => self.rules.push(rule),
                Err(error) => self.errors.push(error.to_string()),
            },
            _ => {}
        }
    }

    fn build(self, path: &Path) -> Result<Exercise, ContentError> {
        if !self.errors.is_empty() {
            return Err(ContentError::new(format!(
                "{:?} exercicio {}: {}",
                path,
                self.id,
                self.errors.join("; ")
            )));
        }

        let module_id = required_meta(&self.meta, "module_id", &self.id, path)?;
        let title = required_meta(&self.meta, "title", &self.id, path)?;
        let difficulty = parse_difficulty(
            self.meta
                .get("difficulty")
                .map(String::as_str)
                .unwrap_or("Beginner"),
        )?;
        let compile_mode = parse_compile_mode(
            self.meta
                .get("compile_mode")
                .map(String::as_str)
                .unwrap_or("SnippetAsMain"),
        )?;
        let prompt = required_block(&self.blocks, "prompt", &self.id, path)?;
        let starter = required_block(&self.blocks, "solution", &self.id, path)?;
        let summary = required_block(&self.blocks, "summary", &self.id, path)?;
        let scaffold = self.blocks.get("scaffold").cloned().unwrap_or_default();

        Ok(Exercise {
            id: self.id,
            module_id,
            title,
            prompt,
            starter: starter.clone(),
            scaffold,
            difficulty,
            guide: SolutionGuide {
                summary,
                solution: starter,
                concepts: self.concepts,
                pitfalls: self.pitfalls,
                docs: self.docs,
            },
            hints: self.hints,
            rules: self.rules,
            compile_mode,
        })
    }
}

fn required_meta(
    meta: &HashMap<String, String>,
    key: &str,
    id: &str,
    path: &Path,
) -> Result<String, ContentError> {
    meta.get(key).cloned().ok_or_else(|| {
        ContentError::new(format!(
            "{:?} exercicio {id}: metadado ausente: {key}",
            path
        ))
    })
}

fn required_block(
    blocks: &HashMap<String, String>,
    key: &str,
    id: &str,
    path: &Path,
) -> Result<String, ContentError> {
    blocks.get(key).cloned().ok_or_else(|| {
        ContentError::new(format!("{:?} exercicio {id}: bloco ausente: {key}", path))
    })
}

fn parse_difficulty(value: &str) -> Result<Difficulty, ContentError> {
    match value {
        "Beginner" | "Iniciante" => Ok(Difficulty::Beginner),
        "Intermediate" | "Intermediario" => Ok(Difficulty::Intermediate),
        "Advanced" | "Avancado" => Ok(Difficulty::Advanced),
        other => Err(ContentError::new(format!("dificuldade invalida: {other}"))),
    }
}

fn parse_compile_mode(value: &str) -> Result<CompileMode, ContentError> {
    match value {
        "Off" => Ok(CompileMode::Off),
        "SnippetAsMain" => Ok(CompileMode::SnippetAsMain),
        "FullProgram" => Ok(CompileMode::FullProgram),
        other => Err(ContentError::new(format!("compile_mode invalido: {other}"))),
    }
}

fn parse_doc(item: &str) -> Option<LearningResource> {
    let (title, rest) = item.strip_prefix('[')?.split_once("](")?;
    let url = rest.strip_suffix(')')?;
    Some(LearningResource {
        title: title.to_string(),
        url: url.to_string(),
    })
}

fn parse_rule(item: &str) -> Result<Rule, ContentError> {
    let parts = item.split('|').map(str::trim).collect::<Vec<_>>();
    if parts.len() < 5 {
        return Err(ContentError::new(format!("regra invalida: {item}")));
    }

    let kind = parts[0];
    let id = parts[1];
    let success = parts[parts.len() - 2];
    let failure = parts[parts.len() - 1];
    let args = &parts[2..parts.len() - 2];

    match kind {
        "required_ast" => Ok(Rule::required_ast(
            id,
            parse_ast_check(args)?,
            success,
            failure,
        )),
        "forbidden_ast" => Ok(Rule::forbidden_ast(
            id,
            parse_ast_check(args)?,
            success,
            failure,
        )),
        "required_pattern" => Ok(Rule::required_pattern(id, args.join("|"), success, failure)),
        "forbidden_pattern" => Ok(Rule::forbidden_pattern(
            id,
            args.join("|"),
            success,
            failure,
        )),
        "min_pattern_count" => {
            if args.len() < 2 {
                return Err(ContentError::new("min_pattern_count precisa pattern e min"));
            }
            let min = args[1]
                .parse::<usize>()
                .map_err(|error| ContentError::new(error.to_string()))?;
            Ok(Rule::min_pattern_count(id, args[0], min, success, failure))
        }
        _ => Err(ContentError::new(format!("tipo de regra invalido: {kind}"))),
    }
}

fn parse_ast_check(args: &[&str]) -> Result<AstCheck, ContentError> {
    let name = args
        .first()
        .ok_or_else(|| ContentError::new("required_ast sem check"))?;

    Ok(match *name {
        "HasLetInitializer" => AstCheck::HasLetInitializer,
        "HasLetInitializerWithInt" => AstCheck::HasLetInitializerWithInt(
            required_arg(args, 1)?
                .parse::<u128>()
                .map_err(|error| ContentError::new(error.to_string()))?,
        ),
        "HasLetInitializerWithPath" => {
            AstCheck::HasLetInitializerWithPath(required_arg(args, 1)?.to_string())
        }
        "HasLetInitializerWithAnyPath" => AstCheck::HasLetInitializerWithAnyPath,
        "HasLetInitializerWithCallPath" => {
            AstCheck::HasLetInitializerWithCallPath(required_arg(args, 1)?.to_string())
        }
        "HasLetInitializerWithCallPathWithIntArg" => {
            AstCheck::HasLetInitializerWithCallPathWithIntArg {
                path: required_arg(args, 1)?.to_string(),
                arg: required_arg(args, 2)?
                    .parse::<u128>()
                    .map_err(|error| ContentError::new(error.to_string()))?,
            }
        }
        "HasLetInitializerWithDeref" => AstCheck::HasLetInitializerWithDeref,
        "HasLetInitializerWithDerefPath" => {
            AstCheck::HasLetInitializerWithDerefPath(required_arg(args, 1)?.to_string())
        }
        "HasLetInitializerWithIf" => AstCheck::HasLetInitializerWithIf,
        "HasStructPatternField" => AstCheck::HasStructPatternField {
            path: required_arg(args, 1)?.to_string(),
            field: required_arg(args, 2)?.to_string(),
        },
        "HasTuplePatternBindingCount" => AstCheck::HasTuplePatternBindingCount {
            count: required_arg(args, 1)?
                .parse::<usize>()
                .map_err(|error| ContentError::new(error.to_string()))?,
        },
        "HasLetMut" => AstCheck::HasLetMut,
        "HasLetType" => AstCheck::HasLetType(required_arg(args, 1)?.to_string()),
        "HasCallPath" => AstCheck::HasCallPath(required_arg(args, 1)?.to_string()),
        "HasCallPathWithIntArg" => AstCheck::HasCallPathWithIntArg {
            path: required_arg(args, 1)?.to_string(),
            arg: required_arg(args, 2)?
                .parse::<u128>()
                .map_err(|error| ContentError::new(error.to_string()))?,
        },
        "HasMethodCall" => AstCheck::HasMethodCall(required_arg(args, 1)?.to_string()),
        "HasMacroCall" => AstCheck::HasMacroCall(required_arg(args, 1)?.to_string()),
        "HasForLoop" => AstCheck::HasForLoop,
        "HasForLoopByReference" => AstCheck::HasForLoopByReference,
        "HasWhileLoop" => AstCheck::HasWhileLoop,
        "HasWhileLetSome" => AstCheck::HasWhileLetSome,
        "HasLoop" => AstCheck::HasLoop,
        "HasIf" => AstCheck::HasIf,
        "HasIfLetSome" => AstCheck::HasIfLetSome,
        "HasMatch" => AstCheck::HasMatch,
        "HasFunction" => AstCheck::HasFunction,
        "HasFunctionParamCount" => AstCheck::HasFunctionParamCount {
            min: required_arg(args, 1)?
                .parse::<usize>()
                .map_err(|error| ContentError::new(error.to_string()))?,
        },
        "HasFunctionReturnType" => AstCheck::HasFunctionReturnType,
        "HasArrayToVec" => AstCheck::HasArrayToVec,
        "HasBinaryAdd" => AstCheck::HasBinaryAdd,
        "MinMethodCallCount" => AstCheck::MinMethodCallCount {
            method: required_arg(args, 1)?.to_string(),
            min: required_arg(args, 2)?
                .parse::<usize>()
                .map_err(|error| ContentError::new(error.to_string()))?,
        },
        other => return Err(ContentError::new(format!("AstCheck invalido: {other}"))),
    })
}

fn required_arg<'a>(args: &'a [&str], index: usize) -> Result<&'a str, ContentError> {
    args.get(index)
        .copied()
        .ok_or_else(|| ContentError::new(format!("argumento ausente na posicao {index}")))
}
