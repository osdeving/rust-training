use crate::domain::{
    AstCheck, CompileMode, Difficulty, Exercise, LearningResource, Rule, SolutionGuide,
    TrainingModule,
};

const MODULE_ID: &str = "collections";

pub fn module() -> TrainingModule {
    TrainingModule {
        id: MODULE_ID,
        title: "Colecoes padrao",
        description: "HashMap, HashSet, VecDeque, stack com Vec e mapas ordenados.",
        exercises: vec![
            hashmap_insert_get(),
            hashset_unique(),
            vecdeque_queue(),
            vec_stack(),
            btreemap_ordered(),
        ],
    }
}

fn hashmap_insert_get() -> Exercise {
    exercise(
        "hashmap-insert-get",
        "Contar pontos com HashMap",
        "Crie um `HashMap`, insira \"Ana\" com 10 pontos e leia o valor com `get`.",
        "use std::collections::HashMap;\nlet mut pontos = HashMap::new();\npontos.insert(\"Ana\", 10);\nlet ana = pontos.get(\"Ana\");",
        "",
        Difficulty::Intermediate,
        guide(
            "`HashMap` associa chaves a valores e permite busca rapida pela chave.",
            "use std::collections::HashMap;\nlet mut pontos = HashMap::new();\npontos.insert(\"Ana\", 10);\nlet ana = pontos.get(\"Ana\");",
            &[
                "`HashMap::new()` cria o mapa vazio.",
                "`insert(chave, valor)` adiciona ou substitui entrada.",
                "`get` retorna `Option<&V>`, porque a chave pode nao existir.",
            ],
            &[
                "O mapa precisa ser mutavel para inserir.",
                "`get` nao retorna o valor direto; trate o `Option` quando usar o resultado.",
            ],
            &[(
                "std::collections::HashMap",
                "https://doc.rust-lang.org/std/collections/struct.HashMap.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "use",
                r"use\s+std::collections::HashMap",
                "Importou HashMap.",
                "Esperava importar HashMap.",
            ),
            Rule::required_ast(
                "insert",
                AstCheck::HasMethodCall("insert"),
                "Usou insert().",
                "Esperava `.insert(...)`.",
            ),
            Rule::required_ast(
                "get",
                AstCheck::HasMethodCall("get"),
                "Usou get().",
                "Esperava `.get(...)`.",
            ),
        ],
        vec![
            "Importe `HashMap` de `std::collections`.",
            "O retorno de `get` e opcional.",
        ],
    )
}

fn hashset_unique() -> Exercise {
    exercise(
        "hashset-unicos",
        "Guardar valores unicos com HashSet",
        "Crie um `HashSet`, insira \"rust\" duas vezes e verifique com `contains`.",
        "use std::collections::HashSet;\nlet mut tags = HashSet::new();\ntags.insert(\"rust\");\ntags.insert(\"rust\");\nlet tem_rust = tags.contains(\"rust\");",
        "",
        Difficulty::Intermediate,
        guide(
            "`HashSet` guarda apenas valores unicos, sem associar valor a uma chave.",
            "use std::collections::HashSet;\nlet mut tags = HashSet::new();\ntags.insert(\"rust\");\ntags.insert(\"rust\");\nlet tem_rust = tags.contains(\"rust\");",
            &[
                "`insert` adiciona o item se ele ainda nao existe.",
                "Inserir o mesmo item duas vezes nao duplica a entrada.",
                "`contains` testa presenca.",
            ],
            &[
                "HashSet nao preserva ordem.",
                "Use HashMap quando precisar associar chave a valor.",
            ],
            &[(
                "std::collections::HashSet",
                "https://doc.rust-lang.org/std/collections/struct.HashSet.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "use",
                r"use\s+std::collections::HashSet",
                "Importou HashSet.",
                "Esperava importar HashSet.",
            ),
            Rule::min_pattern_count(
                "insert-duas",
                r"\.insert\s*\(",
                2,
                "Inseriu pelo menos duas vezes.",
                "Esperava duas chamadas insert.",
            ),
            Rule::required_ast(
                "contains",
                AstCheck::HasMethodCall("contains"),
                "Usou contains().",
                "Esperava `.contains(...)`.",
            ),
        ],
        vec![
            "HashSet e bom para evitar duplicidade.",
            "Use `contains` para testar se um item existe.",
        ],
    )
}

fn vecdeque_queue() -> Exercise {
    exercise(
        "vecdeque-fila",
        "Fila com VecDeque",
        "Use `VecDeque` para inserir no fim com `push_back` e remover do inicio com `pop_front`.",
        "use std::collections::VecDeque;\nlet mut fila = VecDeque::new();\nfila.push_back(\"primeiro\");\nfila.push_back(\"segundo\");\nlet proximo = fila.pop_front();",
        "",
        Difficulty::Intermediate,
        guide(
            "`VecDeque` e uma fila de duas pontas, eficiente para inserir/remover no inicio e fim.",
            "use std::collections::VecDeque;\nlet mut fila = VecDeque::new();\nfila.push_back(\"primeiro\");\nfila.push_back(\"segundo\");\nlet proximo = fila.pop_front();",
            &[
                "`push_back` adiciona no fim.",
                "`pop_front` remove do inicio.",
                "O retorno e `Option<T>` porque a fila pode estar vazia.",
            ],
            &[
                "Usar `Vec::remove(0)` em filas pode ser caro.",
                "Lembre que `pop_front` consome o item removido.",
            ],
            &[(
                "std::collections::VecDeque",
                "https://doc.rust-lang.org/std/collections/struct.VecDeque.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "use",
                r"use\s+std::collections::VecDeque",
                "Importou VecDeque.",
                "Esperava importar VecDeque.",
            ),
            Rule::required_ast(
                "push-back",
                AstCheck::HasMethodCall("push_back"),
                "Usou push_back().",
                "Esperava `.push_back(...)`.",
            ),
            Rule::required_ast(
                "pop-front",
                AstCheck::HasMethodCall("pop_front"),
                "Usou pop_front().",
                "Esperava `.pop_front()`.",
            ),
        ],
        vec![
            "Fila: entra no fim, sai no inicio.",
            "Use `VecDeque::new()`.",
        ],
    )
}

fn vec_stack() -> Exercise {
    exercise(
        "vec-stack",
        "Pilha com Vec",
        "Use um `Vec` como pilha: empilhe com `push` e retire o topo com `pop`.",
        "let mut pilha = Vec::new();\npilha.push(\"base\");\npilha.push(\"topo\");\nlet topo = pilha.pop();",
        "",
        Difficulty::Beginner,
        guide(
            "`Vec` funciona bem como stack quando voce usa o fim como topo.",
            "let mut pilha = Vec::new();\npilha.push(\"base\");\npilha.push(\"topo\");\nlet topo = pilha.pop();",
            &[
                "`push` adiciona no fim.",
                "`pop` remove do fim.",
                "Isso segue LIFO: ultimo a entrar, primeiro a sair.",
            ],
            &[
                "`pop` retorna `Option<T>`.",
                "Nao use indice fixo se a pilha pode estar vazia.",
            ],
            &[(
                "std::vec::Vec",
                "https://doc.rust-lang.org/std/vec/struct.Vec.html",
            )],
        ),
        vec![
            Rule::required_ast(
                "push",
                AstCheck::HasMethodCall("push"),
                "Usou push().",
                "Esperava `.push(...)`.",
            ),
            Rule::required_ast(
                "pop",
                AstCheck::HasMethodCall("pop"),
                "Usou pop().",
                "Esperava `.pop()`.",
            ),
            Rule::required_ast(
                "mut",
                AstCheck::HasLetMut,
                "Declarou pilha mutavel.",
                "Esperava `let mut`.",
            ),
        ],
        vec![
            "Stack com Vec usa o fim como topo.",
            "O retorno de `pop` pode ser None.",
        ],
    )
}

fn btreemap_ordered() -> Exercise {
    exercise(
        "btreemap-ordenado",
        "Mapa ordenado com BTreeMap",
        "Crie um `BTreeMap`, insira duas notas por chave numerica e itere sobre ele com `for`.",
        "use std::collections::BTreeMap;\nlet mut notas = BTreeMap::new();\nnotas.insert(2, \"Bia\");\nnotas.insert(1, \"Ana\");\nfor (ordem, nome) in &notas {\n    println!(\"{ordem}: {nome}\");\n}",
        "use std::collections::BTreeMap;\nlet mut notas = BTreeMap::new();\nnotas.insert(2, \"Bia\");\nnotas.insert(1, \"Ana\");\n",
        Difficulty::Advanced,
        guide(
            "`BTreeMap` mantem as chaves em ordem, diferente de `HashMap`.",
            "use std::collections::BTreeMap;\nlet mut notas = BTreeMap::new();\nnotas.insert(2, \"Bia\");\nnotas.insert(1, \"Ana\");\nfor (ordem, nome) in &notas {\n    println!(\"{ordem}: {nome}\");\n}",
            &[
                "`BTreeMap` ordena por chave.",
                "Iterar por referencia evita consumir o mapa.",
                "O padrao `(ordem, nome)` desempacota cada par chave/valor.",
            ],
            &[
                "Se voce nao precisa de ordem, HashMap costuma ser a escolha mais direta.",
                "A ordem e por chave, nao por ordem de insercao.",
            ],
            &[(
                "std::collections::BTreeMap",
                "https://doc.rust-lang.org/std/collections/struct.BTreeMap.html",
            )],
        ),
        vec![
            Rule::required_pattern(
                "use",
                r"use\s+std::collections::BTreeMap",
                "Importou BTreeMap.",
                "Esperava importar BTreeMap.",
            ),
            Rule::required_ast(
                "for",
                AstCheck::HasForLoop,
                "Iterou com for.",
                "Esperava loop for.",
            ),
            Rule::required_ast(
                "referencia",
                AstCheck::HasForLoopByReference,
                "Iterou por referencia.",
                "Esperava iterar por referencia.",
            ),
        ],
        vec![
            "Use `for (chave, valor) in &mapa`.",
            "BTreeMap ordena pela chave.",
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
