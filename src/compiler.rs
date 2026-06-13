use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::tempdir;

use crate::domain::{CompileMode, CompileReport};

const COMPILE_TIMEOUT: Duration = Duration::from_secs(3);
const INSTALL_URL: &str = "https://rustup.rs";

#[derive(Debug, Clone)]
pub struct RustcStatus {
    pub available: bool,
    pub command: Option<String>,
    pub version: Option<String>,
    pub message: String,
    pub install_url: String,
}

pub fn rustc_status() -> RustcStatus {
    match locate_rustc() {
        Ok(rustc) => match rustc_version(&rustc) {
            Ok(version) => RustcStatus {
                available: true,
                command: Some(display_command(&rustc)),
                version: Some(version.clone()),
                message: format!("Toolchain encontrada: {version}."),
                install_url: INSTALL_URL.to_string(),
            },
            Err(error) => RustcStatus {
                available: false,
                command: Some(display_command(&rustc)),
                version: None,
                message: format!("Encontrei rustc, mas nao consegui executar: {error}."),
                install_url: INSTALL_URL.to_string(),
            },
        },
        Err(error) => RustcStatus {
            available: false,
            command: None,
            version: None,
            message: error.to_string(),
            install_url: INSTALL_URL.to_string(),
        },
    }
}

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
    let rustc = locate_rustc()?;

    std::fs::write(&source_path, source)?;

    let mut child = Command::new(rustc)
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

fn locate_rustc() -> io::Result<PathBuf> {
    for candidate in rustc_candidates() {
        if !is_path_lookup(&candidate) && !candidate.is_file() {
            continue;
        }

        if rustc_version(&candidate).is_ok() {
            return Ok(candidate);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        missing_rustc_message(),
    ))
}

fn rustc_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(path) = env::var("RUST_TRAINING_RUSTC") {
        candidates.push(PathBuf::from(path));
    }

    candidates.push(PathBuf::from(rustc_binary_name()));

    if let Ok(cargo_home) = env::var("CARGO_HOME") {
        candidates.push(
            PathBuf::from(cargo_home)
                .join("bin")
                .join(rustc_binary_name()),
        );
    }

    if let Some(home) = home_dir() {
        candidates.push(home.join(".cargo").join("bin").join(rustc_binary_name()));
    }

    candidates
}

fn rustc_version(command: &Path) -> io::Result<String> {
    let output = Command::new(command).arg("--version").output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ))
    }
}

fn rustc_binary_name() -> &'static str {
    if cfg!(windows) { "rustc.exe" } else { "rustc" }
}

fn is_path_lookup(path: &Path) -> bool {
    path.components().count() == 1
}

fn display_command(path: &Path) -> String {
    if is_path_lookup(path) {
        format!("{} via PATH", path.display())
    } else {
        path.display().to_string()
    }
}

fn home_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(PathBuf::from).or_else(|| {
            match (env::var_os("HOMEDRIVE"), env::var_os("HOMEPATH")) {
                (Some(drive), Some(path)) => {
                    let mut home = PathBuf::from(drive);
                    home.push(path);
                    Some(home)
                }
                _ => None,
            }
        })
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
}

fn missing_rustc_message() -> String {
    if cfg!(windows) {
        "rustc nao foi encontrado. Instale Rust pelo rustup em https://rustup.rs e reabra o app. Se o Rust ja estiver instalado, confira se %USERPROFILE%\\.cargo\\bin esta no PATH ou defina RUST_TRAINING_RUSTC com o caminho do rustc.exe.".to_string()
    } else {
        "rustc nao foi encontrado. Instale Rust pelo rustup em https://rustup.rs e reabra o app. Se o Rust ja estiver instalado, confira se ~/.cargo/bin esta no PATH ou defina RUST_TRAINING_RUSTC com o caminho do rustc.".to_string()
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
