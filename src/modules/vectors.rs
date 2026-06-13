use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "vectors";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Vetores",
        description: "Criação, mutação, acesso e iteração com Vec<T>.",
        exercises: vec![
            vec_new_explicit(),
            vec_inferred_push(),
            vec_macro(),
            vec_from_array(),
            vec_with_capacity(),
            vec_pop(),
            vec_for_ref(),
            vec_iter_map_collect(),
        ],
    }
}

fn vec_new_explicit() -> Exercise {
    Exercise {
        id: "vec-new-explicito",
        module_id: MODULE_ID,
        title: "Criar Vec vazio com tipo explícito",
        prompt: "Crie um Vec vazio com tipo explícito.",
        starter: "let numeros: Vec<i32> = Vec::new();",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for("vec-new-explicito", "let numeros: Vec<i32> = Vec::new();"),
        hints: vec![
            "A anotação de tipo fica na declaração da variável.",
            "O tipo esperado neste exercício é Vec<i32>.",
            "Para um vetor vazio, procure uma função associada de Vec.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "vec-new",
                AstCheck::HasCallPath("Vec::new"),
                "Usou Vec::new().",
                "Esperava encontrar Vec::new().",
            ),
            Rule::required_ast(
                "tipo-explicito",
                AstCheck::HasLetType("Vec"),
                "Declarou o tipo Vec<T> explicitamente.",
                "Este exercício pede uma anotação explícita do tipo Vec<T>.",
            ),
        ],
    }
}

fn vec_inferred_push() -> Exercise {
    Exercise {
        id: "vec-inferido-push",
        module_id: MODULE_ID,
        title: "Inferir tipo no primeiro push",
        prompt: "Crie um Vec sem usar macro e com tipo inferido no primeiro push.",
        starter: "let mut numeros = Vec::new();\nnumeros.push(10);\nnumeros.push(20);",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for(
            "vec-inferido-push",
            "let mut numeros = Vec::new();\nnumeros.push(10);\nnumeros.push(20);",
        ),
        hints: vec![
            "O vetor precisa começar vazio e depois receber valores.",
            "Para inserir depois da criação, a variável precisa ser mutável.",
            "Não escreva Vec<T> na declaração; deixe o primeiro valor inserido decidir o tipo.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "let-mut",
                AstCheck::HasLetMut,
                "Declarou uma variável mutável.",
                "Esperava uma declaração com let mut.",
            ),
            Rule::required_ast(
                "vec-new",
                AstCheck::HasCallPath("Vec::new"),
                "Usou Vec::new().",
                "Esperava encontrar Vec::new().",
            ),
            Rule::required_ast(
                "push-duas-vezes",
                AstCheck::MinMethodCallCount {
                    method: "push",
                    min: 2,
                },
                "Usou push pelo menos duas vezes.",
                "Esperava pelo menos duas chamadas .push(...).",
            ),
            Rule::forbidden_ast(
                "sem-vec-macro",
                AstCheck::HasMacroCall("vec"),
                "Não usou a macro vec!, como pedido.",
                "Você usou vec!, mas este exercício queria Vec::new().",
            ),
            Rule::forbidden_ast(
                "sem-tipo-explicito",
                AstCheck::HasLetType("Vec"),
                "O tipo parece ter sido inferido.",
                "Você anotou Vec<T> explicitamente; este exercício queria inferência pelo push.",
            ),
        ],
    }
}

fn vec_macro() -> Exercise {
    Exercise {
        id: "vec-macro",
        module_id: MODULE_ID,
        title: "Criar Vec com a macro vec!",
        prompt: "Crie um Vec com três números em uma única expressão.",
        starter: "let numeros = vec![1, 2, 3];",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for("vec-macro", "let numeros = vec![1, 2, 3];"),
        hints: vec![
            "Rust tem uma macro própria para literais de vetor.",
            "Macros em Rust usam ! antes dos delimitadores.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "vec-macro",
                AstCheck::HasMacroCall("vec"),
                "Usou a macro vec!.",
                "Esperava encontrar vec![...].",
            ),
            Rule::forbidden_ast(
                "sem-vec-new",
                AstCheck::HasCallPath("Vec::new"),
                "Não usou Vec::new() neste exercício.",
                "Este exercício pede vec!, não Vec::new().",
            ),
        ],
    }
}

fn vec_from_array() -> Exercise {
    Exercise {
        id: "vec-from-array",
        module_id: MODULE_ID,
        title: "Criar Vec a partir de array",
        prompt: "Converta um array em Vec.",
        starter: "let numeros = Vec::from([1, 2, 3]);",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for("vec-from-array", "let numeros = Vec::from([1, 2, 3]);"),
        hints: vec![
            "Você pode converter diretamente um array literal.",
            "Outra opção é chamar um método de conversão em um array.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "array-para-vec",
                AstCheck::HasArrayToVec,
                "Usou uma conversão de array para Vec.",
                "Esperava Vec::from([...]) ou .to_vec().",
            ),
            Rule::forbidden_ast(
                "sem-vec-macro",
                AstCheck::HasMacroCall("vec"),
                "Não usou a macro vec!.",
                "Este exercício queria partir de um array, não da macro vec!.",
            ),
        ],
    }
}

fn vec_with_capacity() -> Exercise {
    Exercise {
        id: "vec-with-capacity",
        module_id: MODULE_ID,
        title: "Reservar capacidade inicial",
        prompt: "Crie um Vec já reservando espaço para três itens e depois preencha-o.",
        starter: "let mut numeros = Vec::with_capacity(3);\nnumeros.push(1);\nnumeros.push(2);\nnumeros.push(3);",
        scaffold: "",
        difficulty: Difficulty::Intermediate,
        guide: guide_for(
            "vec-with-capacity",
            "let mut numeros = Vec::with_capacity(3);\nnumeros.push(1);\nnumeros.push(2);\nnumeros.push(3);",
        ),
        hints: vec![
            "Capacidade reservada não é a mesma coisa que tamanho.",
            "Depois de reservar espaço, ainda é preciso inserir os itens.",
            "Procure a função associada de Vec que recebe uma capacidade inicial.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "with-capacity",
                AstCheck::HasCallPathWithIntArg {
                    path: "Vec::with_capacity",
                    arg: 3,
                },
                "Usou Vec::with_capacity(3).",
                "Esperava encontrar Vec::with_capacity(3).",
            ),
            Rule::required_ast(
                "push-tres-vezes",
                AstCheck::MinMethodCallCount {
                    method: "push",
                    min: 3,
                },
                "Usou push pelo menos três vezes.",
                "Esperava pelo menos três chamadas .push(...).",
            ),
        ],
    }
}

fn vec_pop() -> Exercise {
    Exercise {
        id: "vec-pop",
        module_id: MODULE_ID,
        title: "Remover último item com pop",
        prompt: "Remova o último item de um Vec mutável.",
        starter: "let mut numeros = vec![1, 2, 3];\nlet ultimo = numeros.pop();",
        scaffold: "",
        difficulty: Difficulty::Beginner,
        guide: guide_for(
            "vec-pop",
            "let mut numeros = vec![1, 2, 3];\nlet ultimo = numeros.pop();",
        ),
        hints: vec![
            "A operação altera o vetor.",
            "O método retorna uma opção, porque o vetor pode estar vazio.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "let-mut",
                AstCheck::HasLetMut,
                "Declarou um Vec mutável.",
                "Esperava um Vec mutável com let mut.",
            ),
            Rule::required_ast(
                "pop",
                AstCheck::HasMethodCall("pop"),
                "Usou pop().",
                "Esperava encontrar uma chamada .pop().",
            ),
        ],
    }
}

fn vec_for_ref() -> Exercise {
    Exercise {
        id: "vec-for-referencia",
        module_id: MODULE_ID,
        title: "Percorrer sem consumir o Vec",
        prompt: "Percorra um Vec sem consumi-lo.",
        starter: "let numeros = vec![1, 2, 3];\nfor numero in &numeros {\n    println!(\"{numero}\");\n}",
        scaffold: "",
        difficulty: Difficulty::Intermediate,
        guide: guide_for(
            "vec-for-referencia",
            "let numeros = vec![1, 2, 3];\nfor numero in &numeros {\n    println!(\"{numero}\");\n}",
        ),
        hints: vec![
            "O loop deve emprestar o vetor em vez de tomar posse dele.",
            "Depois do loop, o vetor ainda deveria poder ser usado.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "for",
                AstCheck::HasForLoop,
                "Usou um loop for.",
                "Esperava um loop for.",
            ),
            Rule::required_ast(
                "referencia",
                AstCheck::HasForLoopByReference,
                "Iterou por referência.",
                "Esperava iterar com &vetor para não consumir o Vec.",
            ),
        ],
    }
}

fn vec_iter_map_collect() -> Exercise {
    Exercise {
        id: "vec-map-collect",
        module_id: MODULE_ID,
        title: "Transformar com map e collect",
        prompt: "Crie um novo Vec com os valores dobrados a partir de outro Vec.",
        starter: "let numeros = vec![1, 2, 3];\nlet dobrados: Vec<i32> = numeros.iter().map(|n| n * 2).collect();",
        scaffold: "",
        difficulty: Difficulty::Intermediate,
        guide: guide_for(
            "vec-map-collect",
            "let numeros = vec![1, 2, 3];\nlet dobrados: Vec<i32> = numeros.iter().map(|n| n * 2).collect();",
        ),
        hints: vec![
            "Transforme cada item sem consumir necessariamente o vetor original.",
            "Use uma etapa de transformação antes de materializar o novo Vec.",
            "No fim da cadeia, colete o iterador em um Vec.",
        ],
        compile_mode: CompileMode::SnippetAsMain,
        rules: vec![
            Rule::required_ast(
                "iter",
                AstCheck::HasMethodCall("iter"),
                "Usou iter().",
                "Esperava encontrar .iter().",
            ),
            Rule::required_ast(
                "map",
                AstCheck::HasMethodCall("map"),
                "Usou map().",
                "Esperava encontrar .map(...).",
            ),
            Rule::required_ast(
                "collect",
                AstCheck::HasMethodCall("collect"),
                "Usou collect().",
                "Esperava encontrar .collect().",
            ),
        ],
    }
}

fn guide_for(id: &'static str, solution: &'static str) -> SolutionGuide {
    match id {
        "vec-new-explicito" => guide(
            "Um Vec vazio precisa de uma pista de tipo. Aqui a pista vem da anotacao na variavel.",
            solution,
            &[
                "`Vec::new()` cria um vetor vazio.",
                "Como nao ha valores iniciais, Rust nao infere sozinho o tipo dos itens.",
                "`Vec<i32>` diz que o vetor guardara valores `i32`.",
            ],
            &[
                "Nao confunda `Vec<i32>` com array `[i32; N]`.",
                "A anotacao fica no binding: `let nome: Tipo = valor;`.",
            ],
            &[
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
                ),
            ],
        ),
        "vec-inferido-push" => guide(
            "O vetor comeca vazio, mas o primeiro `push` da ao compilador a informacao de tipo.",
            solution,
            &[
                "`let mut` permite alterar o vetor depois da criacao.",
                "`push` insere itens no fim do Vec.",
                "Depois de `push(10)`, Rust infere que o vetor e de inteiros.",
            ],
            &[
                "Sem `mut`, chamadas como `.push(...)` nao compilam.",
                "Nao use `vec![...]` quando o objetivo e praticar `Vec::new()` mais `push`.",
            ],
            &[
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
                ),
            ],
        ),
        "vec-macro" => guide(
            "A macro `vec!` e o atalho idiomatico para criar um vetor ja preenchido.",
            solution,
            &[
                "Macros em Rust usam `!` antes dos delimitadores.",
                "`vec![1, 2, 3]` cria um `Vec` com os itens informados.",
                "O tipo costuma ser inferido pelos valores ou pelo uso posterior.",
            ],
            &[
                "Use colchetes na macro: `vec![...]`.",
                "Nao misture tipos incompativeis no mesmo Vec.",
            ],
            &[
                (
                    "std::vec! macro",
                    "https://doc.rust-lang.org/std/macro.vec.html",
                ),
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
            ],
        ),
        "vec-from-array" => guide(
            "Arrays tem tamanho fixo; `Vec` cresce dinamicamente. Este exercicio pratica a conversao.",
            solution,
            &[
                "`Vec::from([1, 2, 3])` transforma um array em vetor.",
                "Outra forma comum e `[1, 2, 3].to_vec()`.",
                "Depois de virar Vec, voce pode usar metodos como `push` e `pop`.",
            ],
            &[
                "Array e Vec nao sao o mesmo tipo.",
                "Este exercicio pede partir de array, nao usar diretamente `vec![...]`.",
            ],
            &[
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
                ),
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
            ],
        ),
        "vec-with-capacity" => guide(
            "`with_capacity` reserva memoria, mas nao cria itens. O tamanho ainda comeca em zero.",
            solution,
            &[
                "`Vec::with_capacity(3)` evita realocacoes enquanto voce insere ate tres itens.",
                "Capacidade e espaco reservado; comprimento e quantidade de itens presentes.",
                "`push` ainda e necessario para preencher o vetor.",
            ],
            &[
                "Nao tente acessar indices antes de inserir itens.",
                "Capacidade inicial nao garante que o Vec tera tamanho 3.",
            ],
            &[
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
                ),
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
            ],
        ),
        "vec-pop" => guide(
            "`pop` remove o ultimo item e devolve um `Option`, porque o vetor pode estar vazio.",
            solution,
            &[
                "Remover item muda o Vec, entao a variavel precisa ser `mut`.",
                "`pop()` retorna `Some(valor)` quando havia item.",
                "Se o Vec estiver vazio, o retorno sera `None`.",
            ],
            &[
                "Nao assuma que sempre existe ultimo item.",
                "O valor retornado por `pop` nao e o item direto; e um `Option<T>`.",
            ],
            &[
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
                ),
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
            ],
        ),
        "vec-for-referencia" => guide(
            "Iterar com `&numeros` empresta o Vec. Assim ele continua disponivel depois do loop.",
            solution,
            &[
                "`for numero in &numeros` percorre referencias para os itens.",
                "Isso evita mover o Vec para dentro do loop.",
                "E a forma certa quando voce so precisa ler os valores.",
            ],
            &[
                "`for numero in numeros` consome o vetor em muitos casos.",
                "Use `&mut numeros` apenas quando precisar alterar os itens durante a iteracao.",
            ],
            &[
                (
                    "The Rust Book: Vectors",
                    "https://doc.rust-lang.org/book/ch08-01-vectors.html",
                ),
                (
                    "Iterator trait",
                    "https://doc.rust-lang.org/std/iter/trait.Iterator.html",
                ),
            ],
        ),
        "vec-map-collect" => guide(
            "A cadeia `iter().map(...).collect()` transforma uma sequencia em outra colecao.",
            solution,
            &[
                "`iter()` cria um iterador por referencia.",
                "`map` aplica uma transformacao em cada item.",
                "`collect` materializa o iterador em uma colecao, aqui um `Vec<i32>`.",
            ],
            &[
                "`map` sozinho e lazy: nada e produzido ate consumir ou coletar.",
                "As vezes e preciso anotar o tipo final para o `collect` saber a colecao desejada.",
            ],
            &[
                (
                    "Iterator trait",
                    "https://doc.rust-lang.org/std/iter/trait.Iterator.html",
                ),
                (
                    "std::vec::Vec",
                    "https://doc.rust-lang.org/std/vec/struct.Vec.html",
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
