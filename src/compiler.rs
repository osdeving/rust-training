use std::io;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::tempdir;

use crate::domain::{CompileMode, CompileReport};

const COMPILE_TIMEOUT: Duration = Duration::from_secs(3);

pub fn compile_answer(answer: &str, mode: CompileMode) -> CompileReport {
    match try_compile_answer(answer, mode) {
        Ok(report) => report,
        Err(error) => CompileReport {
            ok: false,
            timed_out: false,
            stderr: format!("Falha ao iniciar rustc: {error}"),
        },
    }
}

fn try_compile_answer(answer: &str, mode: CompileMode) -> io::Result<CompileReport> {
    let source = render_source(answer, mode);
    let dir = tempdir()?;
    let source_path = dir.path().join("answer.rs");
    let metadata_path = dir.path().join("answer.rmeta");

    std::fs::write(&source_path, source)?;

    let mut child = Command::new("rustc")
        .arg("--edition=2024")
        .arg("--emit=metadata")
        .arg("-o")
        .arg(&metadata_path)
        .arg(&source_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let start = Instant::now();

    loop {
        if child.try_wait()?.is_some() {
            let output = child.wait_with_output()?;
            return Ok(CompileReport {
                ok: output.status.success(),
                timed_out: false,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }

        if start.elapsed() >= COMPILE_TIMEOUT {
            let _ = child.kill();
            let output = child.wait_with_output()?;
            return Ok(CompileReport {
                ok: false,
                timed_out: true,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
            });
        }

        thread::sleep(Duration::from_millis(25));
    }
}

fn render_source(answer: &str, mode: CompileMode) -> String {
    match mode {
        CompileMode::Off => answer.to_string(),
        CompileMode::SnippetAsMain => format!("fn main() {{\n{answer}\n}}\n"),
        CompileMode::FullProgram => answer.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_valid_snippet() {
        let report = compile_answer("let x = 1; println!(\"{x}\");", CompileMode::SnippetAsMain);

        assert!(report.ok, "{}", report.stderr);
    }

    #[test]
    fn rejects_invalid_snippet() {
        let report = compile_answer("let x = ;", CompileMode::SnippetAsMain);

        assert!(!report.ok);
        assert!(!report.stderr.is_empty());
    }
}
