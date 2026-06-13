use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "strings";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Strings",
        description: "Construção e composição básica com String.",
        exercises: vec![string_from_push_str(), string_format()],
    }
}

fn string_from_push_str() -> Exercise {
    Exercise {
        id: "string-from-push-str",
        module_id: MODULE_ID,
        title: "Criar String e concatenar texto",
        prompt: "Crie uma String mutável e acrescente outro trecho de texto.",
        starter: "let mut texto = String::from(\"Rust\");\ntexto.push_str(\" é legal\");",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for(
            "string-from-push-str",
            "let mut texto = String::from(\"Rust\");\ntexto.push_str(\" e legal\");",
        ),
        hints: vec![
            "A variável precisa ser mutável, porque o conteúdo será alterado.",
            "Use um método próprio de String para acrescentar uma fatia de texto.",
            "Uma forma comum de criar a String inicial é a partir de um &str.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "string-from",
                AstCheck::HasCallPath("String::from"),
                "Usou String::from(...).",
                "Esperava encontrar String::from(...).",
            ),
            Rule::required_ast(
                "let-mut",
                AstCheck::HasLetMut,
                "Declarou a String como mutável.",
                "Esperava uma declaração com let mut.",
            ),
            Rule::required_ast(
                "push-str",
                AstCheck::HasMethodCall("push_str"),
                "Usou push_str(...).",
                "Esperava encontrar .push_str(...).",
            ),
        ],
    }
}

fn string_format() -> Exercise {
    Exercise {
        id: "string-format",
        module_id: MODULE_ID,
        title: "Compor texto com format!",
        prompt: "Componha uma String interpolando uma variável.",
        starter: "let linguagem = \"Rust\";\nlet mensagem = format!(\"Estou estudando {linguagem}\");",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for(
            "string-format",
            "let linguagem = \"Rust\";\nlet mensagem = format!(\"Estou estudando {linguagem}\");",
        ),
        hints: vec![
            "Rust tem uma macro para montar String com interpolação.",
            "Evite montar o texto com o operador + neste exercício.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "format",
                AstCheck::HasMacroCall("format"),
                "Usou format!.",
                "Esperava encontrar format!(...).",
            ),
            Rule::forbidden_ast(
                "sem-concat-com-mais",
                AstCheck::HasBinaryAdd,
                "Não usou concatenação com +.",
                "Neste exercício, prefira format! em vez de concatenar com +.",
            ),
        ],
    }
}

fn guide_for(id: &'static str, solution: &'static str) -> SolutionGuide {
    match id {
        "string-from-push-str" => guide(
            "`String` e texto alocado e mutavel. `push_str` adiciona uma fatia `&str` no fim.",
            solution,
            &[
                "`String::from(\"Rust\")` cria uma String a partir de um literal `&str`.",
                "A variavel precisa ser `mut` porque `push_str` altera o conteudo.",
                "`push_str` recebe uma fatia de string, nao outra `String` obrigatoriamente.",
            ],
            &[
                "Strings em Rust sao UTF-8; cuidado com assumir tamanho em bytes igual a caracteres.",
                "Sem `mut`, a chamada `.push_str(...)` nao compila.",
            ],
            &[
                (
                    "The Rust Book: Strings",
                    "https://doc.rust-lang.org/book/ch08-02-strings.html",
                ),
                (
                    "std::string::String",
                    "https://doc.rust-lang.org/std/string/struct.String.html",
                ),
            ],
        ),
        "string-format" => guide(
            "`format!` monta uma nova `String` usando a mesma familia de formatacao do `println!`.",
            solution,
            &[
                "`format!(...)` retorna uma `String` em vez de imprimir.",
                "Voce pode interpolar variaveis com `{nome}` dentro do texto.",
                "E uma forma clara de compor mensagens sem encadear `+`.",
            ],
            &[
                "`+` move a String da esquerda e pode deixar o codigo menos claro.",
                "`format!` aloca uma nova String; para loops muito quentes, pense em reutilizar buffer.",
            ],
            &[
                (
                    "std::format! macro",
                    "https://doc.rust-lang.org/std/macro.format.html",
                ),
                (
                    "The Rust Book: Strings",
                    "https://doc.rust-lang.org/book/ch08-02-strings.html",
                ),
            ],
        ),
        _ => guide(
            "Solucao sugerida para o exercicio.",
            solution,
            &[],
            &[],
            &[],
        ),
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
