use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "options-results";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Option e Result",
        description: "Tratamento explicito de ausencia e erro.",
        exercises: vec![option_unwrap_or(), result_question(), if_let_some()],
    }
}

fn option_unwrap_or() -> Exercise {
    exercise(
        "option-unwrap-or",
        "Valor padrao com unwrap_or",
        "Dado `apelido: Option<&str>`, crie `nome` usando `unwrap_or(\"anonimo\")`.",
        "let apelido: Option<&str> = None;\nlet nome = apelido.unwrap_or(\"anonimo\");",
        "let apelido: Option<&str> = None;\n",
        Difficulty::Beginner,
        guide(
            "`unwrap_or` troca `None` por um valor padrao sem causar panic.",
            "let apelido: Option<&str> = None;\nlet nome = apelido.unwrap_or(\"anonimo\");",
            &[
                "`Option` representa presenca ou ausencia.",
                "`unwrap_or` retorna o valor interno ou o fallback.",
                "E mais seguro que `unwrap` quando ausencia e esperada.",
            ],
            &[
                "Evite `unwrap` em fluxo normal de usuario.",
                "O fallback precisa ter tipo compativel com o valor interno.",
            ],
            &[(
                "std::option::Option",
                "https://doc.rust-lang.org/std/option/enum.Option.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "unwrap-or",
                AstCheck::HasMethodCall("unwrap_or"),
                "Usou unwrap_or().",
                "Esperava `.unwrap_or(...)`.",
            ),
            Rule::forbidden_ast(
                "sem-unwrap",
                AstCheck::HasMethodCall("unwrap"),
                "Nao usou unwrap().",
                "Evite `unwrap()` neste exercicio.",
            ),
        ],
        vec![
            "Use um fallback literal.",
            "`unwrap_or` nao falha quando o valor e None.",
        ],
    )
}

fn result_question() -> Exercise {
    exercise(
        "result-question",
        "Propagar erro com ?",
        "Crie `parse_id(texto: &str) -> Result<u32, std::num::ParseIntError>` usando `texto.parse()?`.",
        "fn parse_id(texto: &str) -> Result<u32, std::num::ParseIntError> {\n    let id = texto.parse()?;\n    Ok(id)\n}\nlet id = parse_id(\"42\");",
        "",
        Difficulty::Intermediate,
        guide(
            "`?` reduz boilerplate quando a funcao tambem retorna `Result`.",
            "fn parse_id(texto: &str) -> Result<u32, std::num::ParseIntError> {\n    let id = texto.parse()?;\n    Ok(id)\n}\nlet id = parse_id(\"42\");",
            &[
                "`parse` tenta converter texto em numero.",
                "`?` retorna cedo se houver erro.",
                "`Ok(id)` embrulha o sucesso no Result.",
            ],
            &[
                "`?` so funciona em funcoes que retornam um tipo compativel.",
                "Nao descarte o erro com `unwrap` se o objetivo e propagar falha.",
            ],
            &[(
                "The Rust Book: Result and ?",
                "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "funcao",
                AstCheck::HasFunction,
                "Declarou funcao.",
                "Esperava uma funcao.",
            ),
            Rule::required_pattern(
                "result",
                r"Result\s*<",
                "Retorna Result.",
                "Esperava retorno Result.",
            ),
            Rule::required_pattern("question", r"\?", "Usou ?.", "Esperava usar `?`."),
            Rule::required_pattern("ok", r"Ok\s*\(", "Retornou Ok(...).", "Esperava `Ok(...)`."),
        ],
        vec![
            "O tipo de erro de parse e `std::num::ParseIntError`.",
            "Use `Ok(id)` no final.",
        ],
    )
}

fn if_let_some() -> Exercise {
    exercise(
        "if-let-some",
        "Desestruturar Some com if let",
        "Use `if let Some(nome) = usuario` para preencher `saudacao` quando houver nome.",
        "let usuario = Some(\"Ana\");\nlet mut saudacao = String::from(\"Oi\");\nif let Some(nome) = usuario {\n    saudacao = format!(\"Oi, {nome}\");\n}",
        "let usuario = Some(\"Ana\");\nlet mut saudacao = String::from(\"Oi\");\n",
        Difficulty::Intermediate,
        guide(
            "`if let` e um atalho quando voce quer tratar um padrao especifico.",
            "let usuario = Some(\"Ana\");\nlet mut saudacao = String::from(\"Oi\");\nif let Some(nome) = usuario {\n    saudacao = format!(\"Oi, {nome}\");\n}",
            &[
                "`if let Some(nome) = usuario` roda o bloco apenas se houver valor.",
                "O valor interno fica disponivel como `nome`.",
                "E mais curto que `match` quando so um caso importa.",
            ],
            &[
                "Use `match` se precisar tratar varios casos de forma simetrica.",
                "A variavel alterada dentro do bloco precisa ser `mut`.",
            ],
            &[(
                "The Rust Book: if let",
                "https://doc.rust-lang.org/book/ch06-03-if-let.html",
            )],
        ),
        vec![
            Rule::required_ast("if", AstCheck::HasIf, "Usou if let.", "Esperava if let."),
            Rule::required_pattern(
                "if-let",
                r"if\s+let\s+Some\s*\(",
                "Desestruturou Some.",
                "Esperava `if let Some(...)`.",
            ),
            Rule::required_ast(
                "format",
                AstCheck::HasMacroCall("format"),
                "Usou format!.",
                "Esperava `format!`.",
            ),
        ],
        vec![
            "Use `if let Some(nome) = usuario`.",
            "Atualize `saudacao` dentro do bloco.",
        ],
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
) -> Exercise {
    Exercise {
        id,
        module_id: MODULE_ID,
        title,
        prompt,
        starter: solution,
        scaffold,
        difficulty,
        guide,
        hints,
        rules,
        compile_mode: CompileMode::SnippetAsMain,
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
