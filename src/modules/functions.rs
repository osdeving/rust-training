use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "functions";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Funcoes",
        description: "Parametros, retorno, referencias, tuplas e closures pequenas.",
        exercises: vec![
            fn_value_param(),
            fn_two_params_return(),
            fn_borrow_string(),
            fn_tuple_return(),
            closure_map(),
        ],
    }
}

fn fn_value_param() -> Exercise {
    exercise(
        "fn-parametro-valor",
        "Criar funcao com parametro por valor",
        "Crie uma funcao `dobro` que recebe `n: i32` e retorna `n * 2`.",
        "fn dobro(n: i32) -> i32 {\n    n * 2\n}\nlet valor = dobro(6);",
        "",
        Difficulty::Beginner,
        guide(
            "Funcoes declaram tipos dos parametros e, quando retornam valor, o tipo de retorno.",
            "fn dobro(n: i32) -> i32 {\n    n * 2\n}\nlet valor = dobro(6);",
            &[
                "`fn nome(param: Tipo) -> Retorno` define a assinatura.",
                "A ultima expressao sem `;` vira o retorno.",
                "Tipos escalares como `i32` sao baratos de copiar.",
            ],
            &[
                "Se colocar `;` em `n * 2;`, a funcao retorna `()`.",
                "O tipo do parametro precisa estar explicito.",
            ],
            &[(
                "The Rust Book: Functions",
                "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "funcao",
                AstCheck::HasFunction,
                "Declarou uma funcao.",
                "Esperava uma funcao `fn`.",
            ),
            Rule::required_ast(
                "parametro",
                AstCheck::HasFunctionParamCount { min: 1 },
                "Declarou parametro.",
                "Esperava ao menos um parametro.",
            ),
            Rule::required_ast(
                "retorno",
                AstCheck::HasFunctionReturnType,
                "Declarou tipo de retorno.",
                "Esperava `-> i32`.",
            ),
            Rule::required_pattern(
                "nome",
                r"fn\s+dobro\s*\(",
                "Nomeou a funcao como dobro.",
                "Esperava `fn dobro(...)`.",
            ),
        ],
        vec![
            "A assinatura deve mencionar `n: i32`.",
            "Retorne `n * 2` como ultima expressao.",
        ],
    )
}

fn fn_two_params_return() -> Exercise {
    exercise(
        "fn-dois-parametros",
        "Somar com dois parametros",
        "Crie `somar(a: i32, b: i32) -> i32` e use a funcao para calcular `total`.",
        "fn somar(a: i32, b: i32) -> i32 {\n    a + b\n}\nlet total = somar(2, 3);",
        "",
        Difficulty::Beginner,
        guide(
            "Multiplos parametros sao separados por virgula, cada um com seu tipo.",
            "fn somar(a: i32, b: i32) -> i32 {\n    a + b\n}\nlet total = somar(2, 3);",
            &[
                "Cada parametro precisa ter nome e tipo.",
                "A chamada passa argumentos na mesma ordem da assinatura.",
                "A soma final pode ser a ultima expressao.",
            ],
            &[
                "Nao escreva `a, b: i32`; Rust pede `a: i32, b: i32`.",
                "A ordem dos argumentos importa.",
            ],
            &[(
                "The Rust Book: Functions",
                "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "funcao",
                AstCheck::HasFunction,
                "Declarou uma funcao.",
                "Esperava uma funcao.",
            ),
            Rule::required_ast(
                "dois-parametros",
                AstCheck::HasFunctionParamCount { min: 2 },
                "Declarou dois parametros.",
                "Esperava dois parametros.",
            ),
            Rule::required_ast(
                "soma",
                AstCheck::HasBinaryAdd,
                "Somou os parametros.",
                "Esperava usar `+`.",
            ),
            Rule::required_pattern(
                "chamada",
                r"somar\s*\(",
                "Chamou somar(...).",
                "Esperava chamar a funcao.",
            ),
        ],
        vec![
            "Cada parametro recebe seu proprio `: i32`.",
            "Use a funcao para criar `total`.",
        ],
    )
}

fn fn_borrow_string() -> Exercise {
    exercise(
        "fn-parametro-referencia",
        "Receber String por referencia",
        "Crie `tamanho(texto: &String) -> usize` retornando `texto.len()` sem tomar posse da String.",
        "fn tamanho(texto: &String) -> usize {\n    texto.len()\n}\nlet nome = String::from(\"Ferris\");\nlet total = tamanho(&nome);",
        "let nome = String::from(\"Ferris\");\n",
        Difficulty::Intermediate,
        guide(
            "Parametro por referencia permite ler um valor sem mover a posse para a funcao.",
            "fn tamanho(texto: &String) -> usize {\n    texto.len()\n}\nlet nome = String::from(\"Ferris\");\nlet total = tamanho(&nome);",
            &[
                "`&String` empresta a String.",
                "A chamada usa `&nome` para passar uma referencia.",
                "`len()` retorna tamanho em bytes como `usize`.",
            ],
            &[
                "Preferir `&str` costuma ser mais flexivel, mas aqui o foco e praticar referencia.",
                "Sem `&`, a String seria movida para a funcao.",
            ],
            &[(
                "The Rust Book: References",
                "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
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
                "ref-param",
                r"texto\s*:\s*&\s*String",
                "Recebeu &String.",
                "Esperava parametro `texto: &String`.",
            ),
            Rule::required_ast(
                "len",
                AstCheck::HasMethodCall("len"),
                "Usou len().",
                "Esperava `texto.len()`.",
            ),
            Rule::required_pattern(
                "chamada-ref",
                r"tamanho\s*\(\s*&\s*nome\s*\)",
                "Chamou com &nome.",
                "Esperava chamar `tamanho(&nome)`.",
            ),
        ],
        vec!["Use `&String` na assinatura.", "Na chamada, passe `&nome`."],
    )
}

fn fn_tuple_return() -> Exercise {
    exercise(
        "fn-retorna-tupla",
        "Retornar tupla simples",
        "Crie `min_max(a: i32, b: i32) -> (i32, i32)` usando if/else para retornar menor e maior.",
        "fn min_max(a: i32, b: i32) -> (i32, i32) {\n    if a <= b { (a, b) } else { (b, a) }\n}\nlet par = min_max(9, 4);",
        "",
        Difficulty::Intermediate,
        guide(
            "Tuplas agrupam poucos valores relacionados sem criar struct.",
            "fn min_max(a: i32, b: i32) -> (i32, i32) {\n    if a <= b { (a, b) } else { (b, a) }\n}\nlet par = min_max(9, 4);",
            &[
                "`(i32, i32)` e o tipo de uma tupla com dois inteiros.",
                "`if` pode retornar tuplas diferentes desde que tenham o mesmo tipo.",
                "A funcao retorna menor e maior em uma unica chamada.",
            ],
            &[
                "Os dois ramos do if precisam retornar `(i32, i32)`.",
                "Tuplas grandes ficam pouco legiveis; nesse caso uma struct seria melhor.",
            ],
            &[(
                "The Rust Book: Tuple type",
                "https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type",
            )],
        ),
        vec![
            Rule::required_ast(
                "funcao",
                AstCheck::HasFunction,
                "Declarou funcao.",
                "Esperava uma funcao.",
            ),
            Rule::required_ast("if", AstCheck::HasIf, "Usou if/else.", "Esperava if/else."),
            Rule::required_pattern(
                "retorno-tupla",
                r"->\s*\(\s*i32\s*,\s*i32\s*\)",
                "Declarou retorno de tupla.",
                "Esperava `-> (i32, i32)`.",
            ),
        ],
        vec![
            "O retorno pode ser `(a, b)` ou `(b, a)`.",
            "Use `if a <= b` para decidir a ordem.",
        ],
    )
}

fn closure_map() -> Exercise {
    exercise(
        "closure-map",
        "Usar closure em map",
        "Dado `numeros`, crie `quadrados` usando `.iter().map(|n| n * n).collect()`.",
        "let numeros = vec![1, 2, 3];\nlet quadrados: Vec<i32> = numeros.iter().map(|n| n * n).collect();",
        "let numeros = vec![1, 2, 3];\n",
        Difficulty::Intermediate,
        guide(
            "Closures sao funcoes anonimas, uteis para transformar itens em iteradores.",
            "let numeros = vec![1, 2, 3];\nlet quadrados: Vec<i32> = numeros.iter().map(|n| n * n).collect();",
            &[
                "`|n| n * n` e uma closure com um parametro.",
                "`map` aplica a closure em cada item.",
                "`collect` materializa o resultado em um Vec.",
            ],
            &[
                "`map` e lazy; sem `collect`, nada vira Vec.",
                "Com `.iter()`, `n` e referencia; operadores numericos funcionam aqui por deref coercion.",
            ],
            &[(
                "The Rust Book: Closures",
                "https://doc.rust-lang.org/book/ch13-01-closures.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "iter",
                AstCheck::HasMethodCall("iter"),
                "Usou iter().",
                "Esperava `.iter()`.",
            ),
            Rule::required_ast(
                "map",
                AstCheck::HasMethodCall("map"),
                "Usou map().",
                "Esperava `.map(...)`.",
            ),
            Rule::required_ast(
                "collect",
                AstCheck::HasMethodCall("collect"),
                "Usou collect().",
                "Esperava `.collect()`.",
            ),
            Rule::required_pattern(
                "closure",
                r"\|\s*n\s*\|",
                "Usou closure.",
                "Esperava closure `|n| ...`.",
            ),
        ],
        vec![
            "A closure fica dentro de `map`.",
            "Anote o tipo final se o `collect` precisar de ajuda.",
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
