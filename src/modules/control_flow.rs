use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "control-flow";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Controle de fluxo",
        description: "if, match e as principais formas de loop.",
        exercises: vec![
            if_else_parity(),
            match_option(),
            while_countdown(),
            loop_break_value(),
            nested_for_pairs(),
        ],
    }
}

fn if_else_parity() -> Exercise {
    exercise(
        "if-else-paridade",
        "Classificar paridade com if/else",
        "Crie uma variavel `rotulo` que receba \"par\" se `numero` for par e \"impar\" caso contrario.",
        "let numero = 8;\nlet rotulo = if numero % 2 == 0 { \"par\" } else { \"impar\" };",
        "let numero = 8;\n",
        Difficulty::Beginner,
        guide(
            "`if` em Rust e expressao: ele pode produzir um valor para uma variavel.",
            "let numero = 8;\nlet rotulo = if numero % 2 == 0 { \"par\" } else { \"impar\" };",
            &[
                "Os dois ramos precisam produzir tipos compativeis.",
                "A condicao precisa ser booleana.",
                "Aqui `% 2 == 0` testa se o numero e par.",
            ],
            &[
                "Nao coloque `;` no valor final de cada ramo quando quiser retornar a expressao.",
                "Rust nao converte inteiros para boolean automaticamente.",
            ],
            &[(
                "The Rust Book: if expressions",
                "https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions",
            )],
        ),
        vec![
            Rule::required_ast(
                "if",
                AstCheck::HasIf,
                "Usou if/else.",
                "Esperava uma expressao if/else.",
            ),
            Rule::required_pattern(
                "modulo",
                r"%\s*2\s*==\s*0",
                "Testou divisibilidade por 2.",
                "Esperava testar `numero % 2 == 0`.",
            ),
            Rule::required_pattern(
                "rotulo",
                r"let\s+rotulo\s*=",
                "Criou a variavel rotulo.",
                "Esperava `let rotulo = ...`.",
            ),
        ],
        vec![
            "Use `if condicao { valor } else { outro_valor }`.",
            "O operador `%` calcula o resto da divisao.",
        ],
    )
}

fn match_option() -> Exercise {
    exercise(
        "match-option",
        "Tratar Option com match",
        "Use `match` para criar `texto`: \"valor presente\" quando `entrada` for `Some(_)`, e \"sem valor\" quando for `None`.",
        "let entrada = Some(3);\nlet texto = match entrada {\n    Some(_) => \"valor presente\",\n    None => \"sem valor\",\n};",
        "let entrada = Some(3);\n",
        Difficulty::Beginner,
        guide(
            "`match` obriga voce a tratar todos os casos relevantes de um enum.",
            "let entrada = Some(3);\nlet texto = match entrada {\n    Some(_) => \"valor presente\",\n    None => \"sem valor\",\n};",
            &[
                "`Option<T>` pode ser `Some(valor)` ou `None`.",
                "`_` ignora o valor quando voce nao precisa dele.",
                "Cada braco usa `padrao => expressao`.",
            ],
            &[
                "Nao esqueca o caso `None`.",
                "Os bracos precisam produzir tipos compativeis.",
            ],
            &[(
                "The Rust Book: match",
                "https://doc.rust-lang.org/book/ch06-02-match.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "match",
                AstCheck::HasMatch,
                "Usou match.",
                "Esperava uma expressao match.",
            ),
            Rule::required_pattern(
                "some",
                r"Some\s*\(",
                "Tratou Some(...).",
                "Esperava um braco para Some(...).",
            ),
            Rule::required_pattern(
                "none",
                r"\bNone\b",
                "Tratou None.",
                "Esperava um braco para None.",
            ),
        ],
        vec![
            "O enum Option tem dois casos.",
            "Use `_` se nao precisar do valor dentro de Some.",
        ],
    )
}

fn while_countdown() -> Exercise {
    exercise(
        "while-contador",
        "Contar regressivamente com while",
        "Comece com `contador = 3` e use `while` para reduzir ate zero.",
        "let mut contador = 3;\nwhile contador > 0 {\n    contador -= 1;\n}",
        "",
        Difficulty::Beginner,
        guide(
            "`while` repete enquanto a condicao continuar verdadeira.",
            "let mut contador = 3;\nwhile contador > 0 {\n    contador -= 1;\n}",
            &[
                "O contador precisa ser mutavel para mudar a cada iteracao.",
                "A condicao e reavaliada antes de cada volta.",
                "Reduzir o contador evita loop infinito.",
            ],
            &[
                "Se esquecer `contador -= 1`, o loop nunca termina.",
                "A condicao precisa eventualmente virar falsa.",
            ],
            &[(
                "The Rust Book: loops",
                "https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops",
            )],
        ),
        vec![
            Rule::required_ast(
                "while",
                AstCheck::HasWhileLoop,
                "Usou while.",
                "Esperava um loop while.",
            ),
            Rule::required_ast(
                "mut",
                AstCheck::HasLetMut,
                "Usou variavel mutavel.",
                "O contador precisa ser mutavel.",
            ),
            Rule::required_pattern(
                "decremento",
                r"contador\s*(?:-=|=\s*contador\s*-)",
                "Reduziu o contador.",
                "Esperava decrementar o contador.",
            ),
        ],
        vec![
            "A condicao pode ser `contador > 0`.",
            "Garanta que o corpo altere o contador.",
        ],
    )
}

fn loop_break_value() -> Exercise {
    exercise(
        "loop-break-valor",
        "Retornar valor com loop e break",
        "Use `loop` para incrementar `n` ate 4 e atribua o resultado de `break n * 2` em `dobro`.",
        "let mut n = 0;\nlet dobro = loop {\n    n += 1;\n    if n == 4 {\n        break n * 2;\n    }\n};",
        "let mut n = 0;\n",
        Difficulty::Intermediate,
        guide(
            "`loop` pode retornar valor usando `break expressao`.",
            "let mut n = 0;\nlet dobro = loop {\n    n += 1;\n    if n == 4 {\n        break n * 2;\n    }\n};",
            &[
                "`loop` sozinho e infinito ate encontrar `break`.",
                "`break n * 2` vira o valor da expressao `loop`.",
                "Isso e util quando voce procura um resultado em repeticoes.",
            ],
            &[
                "Sem `break`, o loop nao termina.",
                "Se `break` retorna valor, use o `loop` como expressao: `let x = loop { ... };`.",
            ],
            &[(
                "The Rust Book: returning values from loops",
                "https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops",
            )],
        ),
        vec![
            Rule::required_ast(
                "loop",
                AstCheck::HasLoop,
                "Usou loop.",
                "Esperava `loop { ... }`.",
            ),
            Rule::required_ast(
                "if",
                AstCheck::HasIf,
                "Usou condicao de parada.",
                "Esperava um if para decidir o break.",
            ),
            Rule::required_pattern(
                "break-valor",
                r"break\s+n\s*\*\s*2",
                "Usou break com valor.",
                "Esperava `break n * 2`.",
            ),
        ],
        vec![
            "A expressao `loop` pode ficar do lado direito de `let dobro =`.",
            "Use `break` com o valor que deve sair do loop.",
        ],
    )
}

fn nested_for_pairs() -> Exercise {
    exercise(
        "for-pares-tuplas",
        "Combinar itens com for aninhado",
        "Dado `nomes` e `pontos`, gere `pares` com tuplas `(nome, ponto)` usando dois loops `for`.",
        "let nomes = vec![\"Ana\", \"Bia\"];\nlet pontos = vec![10, 20];\nlet mut pares = Vec::new();\nfor nome in &nomes {\n    for ponto in &pontos {\n        pares.push((*nome, *ponto));\n    }\n}",
        "let nomes = vec![\"Ana\", \"Bia\"];\nlet pontos = vec![10, 20];\nlet mut pares = Vec::new();\n",
        Difficulty::Intermediate,
        guide(
            "Loops aninhados percorrem combinacoes: cada item externo combina com todos os internos.",
            "let nomes = vec![\"Ana\", \"Bia\"];\nlet pontos = vec![10, 20];\nlet mut pares = Vec::new();\nfor nome in &nomes {\n    for ponto in &pontos {\n        pares.push((*nome, *ponto));\n    }\n}",
            &[
                "O primeiro `for` escolhe um nome.",
                "O segundo `for` percorre todos os pontos para aquele nome.",
                "`push` guarda cada combinacao no vetor final.",
            ],
            &[
                "Iterar por referencia evita consumir os vetores originais.",
                "Dois loops aninhados podem crescer rapido: 2 x 3 vira 6 combinacoes.",
            ],
            &[(
                "The Rust Book: for loops",
                "https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for",
            )],
        ),
        vec![
            Rule::required_ast(
                "for",
                AstCheck::HasForLoop,
                "Usou for.",
                "Esperava loop for.",
            ),
            Rule::required_ast(
                "referencia",
                AstCheck::HasForLoopByReference,
                "Iterou por referencia.",
                "Esperava iterar com referencia.",
            ),
            Rule::required_ast(
                "push",
                AstCheck::HasMethodCall("push"),
                "Guardou combinacoes com push.",
                "Esperava usar push em pares.",
            ),
            Rule::min_pattern_count(
                "dois-for",
                r"\bfor\b",
                2,
                "Usou dois loops for.",
                "Esperava dois loops for.",
            ),
        ],
        vec![
            "Use `for nome in &nomes` e dentro dele `for ponto in &pontos`.",
            "Guarde uma tupla no vetor `pares`.",
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
