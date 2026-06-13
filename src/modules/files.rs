use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "files";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Arquivos",
        description: "Leitura, escrita, append e caminhos com a biblioteca padrao.",
        exercises: vec![
            fs_read_to_string(),
            fs_write_file(),
            open_options_append(),
            path_exists(),
        ],
    }
}

fn fs_read_to_string() -> Exercise {
    exercise(
        "fs-read-to-string",
        "Ler arquivo como String",
        "Crie `conteudo` lendo `notas.txt` com `std::fs::read_to_string` e propague erro com `?`.",
        "fn main() -> std::io::Result<()> {\n    use std::fs;\n    let conteudo = fs::read_to_string(\"notas.txt\")?;\n    Ok(())\n}",
        "fn main() -> std::io::Result<()> {\n    use std::fs;\n    \n    Ok(())\n}\n",
        Difficulty::Intermediate,
        guide(
            "`read_to_string` e o caminho curto para ler um arquivo textual inteiro.",
            "fn main() -> std::io::Result<()> {\n    use std::fs;\n    let conteudo = fs::read_to_string(\"notas.txt\")?;\n    Ok(())\n}",
            &[
                "`fs::read_to_string` retorna `Result<String, io::Error>`.",
                "`?` propaga o erro para a funcao chamadora.",
                "E adequado para arquivos pequenos ou medios de texto.",
            ],
            &[
                "Este snippet precisa estar em uma funcao que retorna `Result`.",
                "Para arquivos grandes, pense em leitura incremental.",
            ],
            &[(
                "std::fs::read_to_string",
                "https://doc.rust-lang.org/std/fs/fn.read_to_string.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "read",
                r"fs::read_to_string\s*\(",
                "Usou fs::read_to_string.",
                "Esperava `fs::read_to_string(...)`.",
            ),
            Rule::required_pattern(
                "question",
                r"\?",
                "Propagou erro com ?.",
                "Esperava usar `?`.",
            ),
        ],
        vec!["Use `use std::fs;`.", "O operador `?` propaga o erro."],
        CompileMode::FullProgram,
        "fn main() -> std::io::Result<()> {\n    use std::fs;\n    let conteudo = fs::read_to_string(\"notas.txt\")?;\n    Ok(())\n}\n",
    )
}

fn fs_write_file() -> Exercise {
    exercise(
        "fs-write",
        "Escrever arquivo",
        "Escreva \"feito\" em `saida.txt` usando `std::fs::write` e propague erro com `?`.",
        "fn main() -> std::io::Result<()> {\n    std::fs::write(\"saida.txt\", \"feito\")?;\n    Ok(())\n}",
        "fn main() -> std::io::Result<()> {\n    \n    Ok(())\n}\n",
        Difficulty::Intermediate,
        guide(
            "`std::fs::write` cria ou substitui um arquivo com todo o conteudo informado.",
            "fn main() -> std::io::Result<()> {\n    std::fs::write(\"saida.txt\", \"feito\")?;\n    Ok(())\n}",
            &[
                "`write` recebe caminho e conteudo.",
                "Se o arquivo existir, ele sera substituido.",
                "`?` propaga falhas de permissao, caminho invalido, etc.",
            ],
            &[
                "Nao use `write` quando precisa acrescentar ao fim; use OpenOptions.",
                "Funcoes com `?` precisam retornar `Result` compativel.",
            ],
            &[(
                "std::fs::write",
                "https://doc.rust-lang.org/std/fs/fn.write.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "write",
                r"(?:std::)?fs::write\s*\(",
                "Usou fs::write.",
                "Esperava `std::fs::write(...)`.",
            ),
            Rule::required_pattern(
                "question",
                r"\?",
                "Propagou erro com ?.",
                "Esperava usar `?`.",
            ),
        ],
        vec![
            "`std::fs::write(caminho, conteudo)` substitui o arquivo.",
            "Retorne `Ok(())` no final.",
        ],
        CompileMode::FullProgram,
        "fn main() -> std::io::Result<()> {\n    std::fs::write(\"saida.txt\", \"feito\")?;\n    Ok(())\n}\n",
    )
}

fn open_options_append() -> Exercise {
    exercise(
        "openoptions-append",
        "Adicionar ao fim com OpenOptions",
        "Abra `log.txt` para append criando o arquivo se necessario.",
        "fn main() -> std::io::Result<()> {\n    use std::fs::OpenOptions;\n    let arquivo = OpenOptions::new()\n        .create(true)\n        .append(true)\n        .open(\"log.txt\")?;\n    Ok(())\n}",
        "fn main() -> std::io::Result<()> {\n    use std::fs::OpenOptions;\n    \n    Ok(())\n}\n",
        Difficulty::Advanced,
        guide(
            "`OpenOptions` configura como um arquivo deve ser aberto.",
            "fn main() -> std::io::Result<()> {\n    use std::fs::OpenOptions;\n    let arquivo = OpenOptions::new()\n        .create(true)\n        .append(true)\n        .open(\"log.txt\")?;\n    Ok(())\n}",
            &[
                "`create(true)` cria se nao existir.",
                "`append(true)` posiciona escrita no fim.",
                "`open` retorna `Result<File>`.",
            ],
            &[
                "Append nao apaga conteudo anterior.",
                "Sem `create(true)`, abrir arquivo inexistente falha.",
            ],
            &[(
                "std::fs::OpenOptions",
                "https://doc.rust-lang.org/std/fs/struct.OpenOptions.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "openoptions",
                r"OpenOptions::new\s*\(",
                "Usou OpenOptions::new().",
                "Esperava `OpenOptions::new()`.",
            ),
            Rule::required_ast(
                "create",
                AstCheck::HasMethodCall("create"),
                "Configurou create().",
                "Esperava `.create(true)`.",
            ),
            Rule::required_ast(
                "append",
                AstCheck::HasMethodCall("append"),
                "Configurou append().",
                "Esperava `.append(true)`.",
            ),
            Rule::required_ast(
                "open",
                AstCheck::HasMethodCall("open"),
                "Chamou open().",
                "Esperava `.open(...)`.",
            ),
        ],
        vec![
            "Encadeie os metodos em `OpenOptions::new()`.",
            "Use `?` para propagar o erro de abertura.",
        ],
        CompileMode::FullProgram,
        "fn main() -> std::io::Result<()> {\n    use std::fs::OpenOptions;\n    let arquivo = OpenOptions::new()\n        .create(true)\n        .append(true)\n        .open(\"log.txt\")?;\n    Ok(())\n}\n",
    )
}

fn path_exists() -> Exercise {
    exercise(
        "path-exists",
        "Checar se caminho existe",
        "Crie `existe` usando `Path::new(\"Cargo.toml\").exists()`.",
        "use std::path::Path;\nlet existe = Path::new(\"Cargo.toml\").exists();",
        "",
        Difficulty::Beginner,
        guide(
            "`Path` representa caminhos de forma portavel.",
            "use std::path::Path;\nlet existe = Path::new(\"Cargo.toml\").exists();",
            &[
                "`Path::new` cria uma referencia de caminho.",
                "`exists` consulta o sistema de arquivos e retorna bool.",
                "E util para fluxos simples antes de abrir ou criar arquivos.",
            ],
            &[
                "Entre a checagem e o uso, o arquivo ainda pode mudar.",
                "Para operacoes reais, trate erros da operacao final.",
            ],
            &[(
                "std::path::Path",
                "https://doc.rust-lang.org/std/path/struct.Path.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "path",
                r"Path::new\s*\(",
                "Usou Path::new().",
                "Esperava `Path::new(...)`.",
            ),
            Rule::required_ast(
                "exists",
                AstCheck::HasMethodCall("exists"),
                "Usou exists().",
                "Esperava `.exists()`.",
            ),
        ],
        vec!["Importe `std::path::Path`.", "`exists()` retorna bool."],
        CompileMode::SnippetAsMain,
        "use std::path::Path;\nlet existe = Path::new(\"Cargo.toml\").exists();",
    )
}

fn exercise(
    id: &'static str,
    title: &'static str,
    prompt: &'static str,
    solution: &'static str,
    scaffold: &'static str,
    difficulty: Difficulty,
    guide: SolutionGuide,
    rules: Vec<Rule>,
    hints: Vec<&'static str>,
    compile_mode: CompileMode,
    starter: &'static str,
) -> Exercise {
    Exercise {
        id,
        module_id: MODULE_ID,
        title,
        prompt,
        starter,
        scaffold,
        difficulty,
        guide: SolutionGuide { solution, ..guide },
        hints,
        rules,
        compile_mode,
    }
}

fn guide(
    summary: &'static str,
    solution: &'static str,
    concepts: &[&'static str],
    pitfalls: &[&'static str],
    docs: &[(&'static str, &'static str)],
) -> SolutionGuide {
    SolutionGuide {
        summary,
        solution,
        concepts: concepts.to_vec(),
        pitfalls: pitfalls.to_vec(),
        docs: docs
            .iter()
            .map(|(title, url)| LearningResource { title, url })
            .collect(),
    }
}
