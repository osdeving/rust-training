use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use rust_training::{Exercise, ValidationReport, all_modules, evaluate, find_exercise};

fn main() -> ExitCode {
    let args = env::args().skip(1).collect::<Vec<_>>();

    match args.as_slice() {
        [] => {
            print_help();
            ExitCode::SUCCESS
        }
        [command] if command == "help" || command == "--help" || command == "-h" => {
            print_help();
            ExitCode::SUCCESS
        }
        [command] if command == "list" => {
            list_exercises();
            ExitCode::SUCCESS
        }
        [command, id] if command == "show" => match find_exercise(id) {
            Some(exercise) => {
                print_exercise(&exercise);
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("Exercicio nao encontrado: {id}");
                ExitCode::from(1)
            }
        },
        [command, id] if command == "hint" => match find_exercise(id) {
            Some(exercise) => {
                if print_hint(&exercise, HintRequest::One(1)) {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::from(1)
                }
            }
            None => {
                eprintln!("Exercicio nao encontrado: {id}");
                ExitCode::from(1)
            }
        },
        [command, id, request] if command == "hint" => match find_exercise(id) {
            Some(exercise) => match HintRequest::parse(request) {
                Ok(request) => {
                    if print_hint(&exercise, request) {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::from(1)
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(2)
                }
            },
            None => {
                eprintln!("Exercicio nao encontrado: {id}");
                ExitCode::from(1)
            }
        },
        [command, id] if command == "check" => check_answer(id, None),
        [command, id, path] if command == "check" => check_answer(id, Some(path)),
        _ => {
            eprintln!("Comando invalido.\n");
            print_help();
            ExitCode::from(2)
        }
    }
}

fn print_help() {
    println!(
        "\
rust-training

Uso:
  rust-training list
  rust-training show <exercise-id>
  rust-training hint <exercise-id> [n|all]
  rust-training check <exercise-id> <arquivo.rs>
  rust-training check <exercise-id> < resposta.rs

Durante desenvolvimento, use os mesmos comandos com:
  cargo run -- <comando>

Exemplo:
  rust-training show vec-inferido-push
  rust-training hint vec-inferido-push 2
  rust-training check vec-inferido-push exemplos/minha_resposta.rs
"
    );
}

fn list_exercises() {
    for module in all_modules() {
        println!("{} - {}", module.id, module.title);
        println!("  {}", module.description);

        for exercise in module.exercises {
            println!("  {:<24} {}", exercise.id, exercise.title);
        }

        println!();
    }
}

fn print_exercise(exercise: &Exercise) {
    println!("{} [{}]", exercise.title, exercise.id);
    println!();
    println!("{}", exercise.prompt);

    if !exercise.hints.is_empty() {
        println!();
        println!("Dica: rust-training hint {}", exercise.id);
    }
}

fn print_hint(exercise: &Exercise, request: HintRequest) -> bool {
    if exercise.hints.is_empty() {
        eprintln!("Este exercicio nao tem dicas cadastradas.");
        return false;
    }

    match request {
        HintRequest::One(number) => {
            let Some(hint) = exercise.hints.get(number.saturating_sub(1)) else {
                eprintln!(
                    "Dica nao encontrada. Este exercicio tem {} dica(s).",
                    exercise.hints.len()
                );
                return false;
            };

            println!("Dica {number}/{}:", exercise.hints.len());
            println!("{hint}");
            true
        }
        HintRequest::All => {
            for (index, hint) in exercise.hints.iter().enumerate() {
                println!("Dica {}/{}:", index + 1, exercise.hints.len());
                println!("{hint}");

                if index + 1 < exercise.hints.len() {
                    println!();
                }
            }
            true
        }
    }
}

fn check_answer(id: &str, path: Option<&String>) -> ExitCode {
    let Some(exercise) = find_exercise(id) else {
        eprintln!("Exercicio nao encontrado: {id}");
        return ExitCode::from(1);
    };

    let answer = match read_answer(path) {
        Ok(answer) => answer,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(1);
        }
    };

    let report = evaluate(&exercise, &answer);
    print_report(&report);

    if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

enum HintRequest {
    One(usize),
    All,
}

impl HintRequest {
    fn parse(input: &str) -> Result<Self, String> {
        if input == "all" {
            return Ok(Self::All);
        }

        let number = input
            .parse::<usize>()
            .map_err(|_| "A dica deve ser um numero ou 'all'.".to_string())?;

        if number == 0 {
            return Err("A dica deve comecar em 1.".to_string());
        }

        Ok(Self::One(number))
    }
}

fn read_answer(path: Option<&String>) -> Result<String, String> {
    match path {
        Some(path) => {
            fs::read_to_string(path).map_err(|error| format!("Erro lendo {path}: {error}"))
        }
        None => {
            let mut answer = String::new();
            io::stdin()
                .read_to_string(&mut answer)
                .map_err(|error| format!("Erro lendo stdin: {error}"))?;
            Ok(answer)
        }
    }
}

fn print_report(report: &ValidationReport) {
    println!("Resultado do exercicio: {}", report.exercise_id);
    println!();

    if let Some(compile) = &report.compile {
        if compile.ok {
            println!("OK Codigo compila.");
        } else if compile.timed_out {
            println!("FAIL Compilacao excedeu o tempo limite.");
        } else {
            println!("FAIL Codigo nao compila.");
        }

        if !compile.stderr.is_empty() {
            println!();
            println!("{}", compile.stderr);
            println!();
        }
    }

    for check in &report.checks {
        let marker = if check.ok { "OK" } else { "FAIL" };
        println!("{marker} {}", check.message);

        if let Some(detail) = &check.detail {
            println!("  {detail}");
        }
    }

    println!();

    if report.passed {
        println!("Status: aprovado");
    } else {
        println!("Status: precisa ajustar");
    }
}
