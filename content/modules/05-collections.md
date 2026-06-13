id: collections
title: Colecoes padrao
description: HashMap, HashSet, VecDeque, stack com Vec e mapas ordenados.

## hashmap-insert-get
module_id: collections
title: Contar pontos com HashMap
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um HashMap, insira "Ana" com 10 pontos e leia o valor com get.
```
```summary
HashMap associa chaves a valores e permite busca rapida pela chave.
```
```solution
use std::collections::HashMap;
let mut pontos = HashMap::new();
pontos.insert("Ana", 10);
let ana = pontos.get("Ana");
```
### Hints
- Importe HashMap de std::collections.
### Concepts
- insert adiciona ou substitui entrada.
- get retorna Option<&V>.
### Pitfalls
- O mapa precisa ser mutavel para inserir.
### Docs
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
### Rules
- required_pattern|use|use\s+std::collections::HashMap|Importou HashMap.|Esperava importar HashMap.
- required_ast|insert|HasMethodCall|insert|Usou insert().|Esperava .insert(...).
- required_ast|get|HasMethodCall|get|Usou get().|Esperava .get(...).

## hashset-unicos
module_id: collections
title: Guardar valores unicos com HashSet
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um HashSet, insira "rust" duas vezes e verifique com contains.
```
```summary
HashSet guarda apenas valores unicos, sem associar valor a uma chave.
```
```solution
use std::collections::HashSet;
let mut tags = HashSet::new();
tags.insert("rust");
tags.insert("rust");
let tem_rust = tags.contains("rust");
```
### Hints
- HashSet e bom para evitar duplicidade.
### Concepts
- Inserir o mesmo item duas vezes nao duplica a entrada.
### Pitfalls
- HashSet nao preserva ordem.
### Docs
- [std::collections::HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
### Rules
- required_pattern|use|use\s+std::collections::HashSet|Importou HashSet.|Esperava importar HashSet.
- min_pattern_count|insert-duas|\.insert\s*\(|2|Inseriu pelo menos duas vezes.|Esperava duas chamadas insert.
- required_ast|contains|HasMethodCall|contains|Usou contains().|Esperava .contains(...).

## vecdeque-fila
module_id: collections
title: Fila com VecDeque
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Use VecDeque para inserir no fim com push_back e remover do inicio com pop_front.
```
```summary
VecDeque e uma fila de duas pontas, eficiente para inserir/remover no inicio e fim.
```
```solution
use std::collections::VecDeque;
let mut fila = VecDeque::new();
fila.push_back("primeiro");
fila.push_back("segundo");
let proximo = fila.pop_front();
```
### Hints
- Fila: entra no fim, sai no inicio.
### Concepts
- push_back adiciona no fim.
- pop_front remove do inicio.
### Pitfalls
- Vec::remove(0) em filas pode ser caro.
### Docs
- [std::collections::VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
### Rules
- required_pattern|use|use\s+std::collections::VecDeque|Importou VecDeque.|Esperava importar VecDeque.
- required_ast|push-back|HasMethodCall|push_back|Usou push_back().|Esperava .push_back(...).
- required_ast|pop-front|HasMethodCall|pop_front|Usou pop_front().|Esperava .pop_front().

## vec-stack
module_id: collections
title: Pilha com Vec
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Use um Vec como pilha: empilhe com push e retire o topo com pop.
```
```summary
Vec funciona bem como stack quando voce usa o fim como topo.
```
```solution
let mut pilha = Vec::new();
pilha.push("base");
pilha.push("topo");
let topo = pilha.pop();
```
### Hints
- Stack com Vec usa o fim como topo.
### Concepts
- LIFO: ultimo a entrar, primeiro a sair.
### Pitfalls
- pop retorna Option<T>.
### Docs
- [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
### Rules
- required_ast|push|HasMethodCall|push|Usou push().|Esperava .push(...).
- required_ast|pop|HasMethodCall|pop|Usou pop().|Esperava .pop().
- required_ast|mut|HasLetMut|Declarou pilha mutavel.|Esperava let mut.

## btreemap-ordenado
module_id: collections
title: Mapa ordenado com BTreeMap
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie um BTreeMap, insira duas notas por chave numerica e itere sobre ele com for.
```
```scaffold
use std::collections::BTreeMap;
let mut notas = BTreeMap::new();
notas.insert(2, "Bia");
notas.insert(1, "Ana");
```
```summary
BTreeMap mantem as chaves em ordem, diferente de HashMap.
```
```solution
use std::collections::BTreeMap;
let mut notas = BTreeMap::new();
notas.insert(2, "Bia");
notas.insert(1, "Ana");
for (ordem, nome) in &notas {
    println!("{ordem}: {nome}");
}
```
### Hints
- Use for (chave, valor) in &mapa.
### Concepts
- BTreeMap ordena por chave.
### Pitfalls
- A ordem e por chave, nao por insercao.
### Docs
- [std::collections::BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
### Rules
- required_pattern|use|use\s+std::collections::BTreeMap|Importou BTreeMap.|Esperava importar BTreeMap.
- required_ast|for|HasForLoop|Iterou com for.|Esperava loop for.
- required_ast|referencia|HasForLoopByReference|Iterou por referencia.|Esperava iterar por referencia.
