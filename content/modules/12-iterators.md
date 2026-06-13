id: iterators
title: Iteradores
description: iter, iter_mut, into_iter, adaptadores e consumidores idiomaticos.

## iter-sum
module_id: iterators
title: Somar valores com iter().sum()
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie total usando numeros.iter().sum().
```
```scaffold
let numeros = vec![1, 2, 3];
```
```summary
sum consome um iterador e acumula os itens em um valor.
```
```solution
let numeros = vec![1, 2, 3];
let total: i32 = numeros.iter().sum();
```
### Hints
- Anote o tipo de total para guiar sum.
### Concepts
- iter empresta os itens; sum faz o consumo do iterador.
### Pitfalls
- Sem tipo alvo, sum pode precisar de ajuda da inferencia.
### Docs
- [Iterator::sum](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
### Rules
- required_ast|iter|HasMethodCall|iter|Criou iterador.|Esperava iter.
- required_ast|sum|HasMethodCall|sum|Somou itens.|Esperava sum.
- required_ast|tipo|HasLetType|i32|Indicou tipo do resultado.|Esperava tipo i32.

## iter-enumerate
module_id: iterators
title: Usar enumerate para indice e valor
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado nomes, crie pares usando iter().enumerate().collect().
```
```scaffold
let nomes = vec!["Ana", "Bia"];
```
```summary
enumerate transforma cada item em uma tupla com indice e valor.
```
```solution
let nomes = vec!["Ana", "Bia"];
let pares: Vec<(usize, &&str)> = nomes.iter().enumerate().collect();
```
### Hints
- enumerate vem depois de iter.
### Concepts
- O indice produzido por enumerate e usize.
### Pitfalls
- iter sobre Vec<&str> produz &&str.
### Docs
- [Iterator::enumerate](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate)
### Rules
- required_ast|iter|HasMethodCall|iter|Criou iterador.|Esperava iter.
- required_ast|enumerate|HasMethodCall|enumerate|Usou enumerate.|Esperava enumerate.
- required_ast|collect|HasMethodCall|collect|Coletou resultado.|Esperava collect.

## iter-filter-map
module_id: iterators
title: Filtrar e transformar com filter_map
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado textos, crie numeros: Vec<i32> usando iter, filter_map e parse().ok().
```
```scaffold
let textos = vec!["1", "x", "3"];
```
```summary
filter_map remove None e transforma Some ao mesmo tempo.
```
```solution
let textos = vec!["1", "x", "3"];
let numeros: Vec<i32> = textos.iter().filter_map(|texto| texto.parse().ok()).collect();
```
### Hints
- parse retorna Result; ok transforma em Option.
### Concepts
- filter_map e comum para parsing parcial.
### Pitfalls
- map sozinho manteria os erros.
### Docs
- [Iterator::filter_map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)
### Rules
- required_ast|filter-map|HasMethodCall|filter_map|Usou filter_map.|Esperava filter_map.
- required_ast|parse|HasMethodCall|parse|Tentou parsear.|Esperava parse.
- required_ast|ok|HasMethodCall|ok|Converteu Result em Option.|Esperava ok.
- required_ast|collect|HasMethodCall|collect|Coletou numeros.|Esperava collect.

## iter-find
module_id: iterators
title: Encontrar primeiro item
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie primeiro_par usando iter().find para achar numero par.
```
```scaffold
let numeros = vec![1, 3, 4, 6];
```
```summary
find para no primeiro item que satisfaz a condicao.
```
```solution
let numeros = vec![1, 3, 4, 6];
let primeiro_par = numeros.iter().find(|n| **n % 2 == 0);
```
### Hints
- A closure recebe uma referencia para o item do iterador.
### Concepts
- find retorna Option.
### Pitfalls
- Cuidado com o nivel de referencia dentro da closure.
### Docs
- [Iterator::find](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
### Rules
- required_ast|iter|HasMethodCall|iter|Criou iterador.|Esperava iter.
- required_ast|find|HasMethodCall|find|Buscou primeiro item.|Esperava find.
- required_pattern|par|%\s*2\s*==\s*0|Checou paridade.|Esperava testar par.

## iter-any-all
module_id: iterators
title: Usar any e all
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie tem_negativo com any e todos_positivos com all.
```
```scaffold
let numeros = vec![1, 2, 3];
```
```summary
any verifica se algum item passa; all verifica se todos passam.
```
```solution
let numeros = vec![1, 2, 3];
let tem_negativo = numeros.iter().any(|n| *n < 0);
let todos_positivos = numeros.iter().all(|n| *n > 0);
```
### Hints
- Use duas consultas separadas.
### Concepts
- any e all retornam bool.
### Pitfalls
- all em iterador vazio retorna true.
### Docs
- [Iterator::any](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)
- [Iterator::all](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)
### Rules
- required_ast|any|HasMethodCall|any|Usou any.|Esperava any.
- required_ast|all|HasMethodCall|all|Usou all.|Esperava all.
- required_ast|iter|MinMethodCallCount|iter|2|Criou dois iteradores.|Esperava iterar duas vezes.

## iter-zip
module_id: iterators
title: Combinar duas colecoes com zip
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado nomes e notas, crie pares usando nomes.iter().zip(notas.iter()).collect().
```
```scaffold
let nomes = vec!["Ana", "Bia"];
let notas = vec![9, 8];
```
```summary
zip combina dois iteradores em pares ate o menor terminar.
```
```solution
let nomes = vec!["Ana", "Bia"];
let notas = vec![9, 8];
let pares: Vec<(&&str, &i32)> = nomes.iter().zip(notas.iter()).collect();
```
### Hints
- Chame zip no primeiro iterador.
### Concepts
- zip e util para percorrer colecoes alinhadas.
### Pitfalls
- Itens extras do iterador maior sao ignorados.
### Docs
- [Iterator::zip](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
### Rules
- required_ast|zip|HasMethodCall|zip|Combinou iteradores.|Esperava zip.
- required_ast|iter|MinMethodCallCount|iter|2|Iterou duas colecoes.|Esperava dois iter.
- required_ast|collect|HasMethodCall|collect|Coletou pares.|Esperava collect.

## iter-fold
module_id: iterators
title: Acumular resultado com fold
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie produto usando iter().fold(1, |acc, n| acc * n).
```
```scaffold
let numeros = vec![2, 3, 4];
```
```summary
fold permite controlar o acumulador e a regra de combinacao.
```
```solution
let numeros = vec![2, 3, 4];
let produto = numeros.iter().fold(1, |acc, n| acc * n);
```
### Hints
- O primeiro argumento e o acumulador inicial.
### Concepts
- fold e mais geral que sum e product.
### Pitfalls
- Escolha um acumulador inicial coerente.
### Docs
- [Iterator::fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
### Rules
- required_ast|iter|HasMethodCall|iter|Criou iterador.|Esperava iter.
- required_ast|fold|HasMethodCall|fold|Usou fold.|Esperava fold.
- required_pattern|multiplica|acc\s*\*\s*n|Multiplicou acumulador.|Esperava acc * n.

## iter-partition
module_id: iterators
title: Separar itens com partition
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie (pares, impares): (Vec<i32>, Vec<i32>) usando into_iter().partition.
```
```scaffold
let numeros = vec![1, 2, 3, 4];
```
```summary
partition divide um iterador em duas colecoes conforme um predicado.
```
```solution
let numeros = vec![1, 2, 3, 4];
let (pares, impares): (Vec<i32>, Vec<i32>) = numeros.into_iter().partition(|n| n % 2 == 0);
```
### Hints
- A anotacao de tipo ajuda partition a saber o destino.
### Concepts
- into_iter consome o Vec e entrega valores.
### Pitfalls
- Depois de into_iter, numeros foi movido.
### Docs
- [Iterator::partition](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition)
### Rules
- required_ast|into-iter|HasMethodCall|into_iter|Consumiu colecao.|Esperava into_iter.
- required_ast|partition|HasMethodCall|partition|Separou itens.|Esperava partition.
- required_pattern|tipo|\(Vec\s*<\s*i32\s*>\s*,\s*Vec\s*<\s*i32\s*>\)|Anotou tipo das colecoes.|Esperava tipo do par.

## iter-copied-cloned
module_id: iterators
title: Diferenca entre copied e cloned
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numeros e nomes, crie valores com copied e nomes_clone com cloned.
```
```scaffold
let numeros = vec![1, 2, 3];
let nomes = vec![String::from("Ana")];
```
```summary
copied copia tipos Copy; cloned chama Clone para produzir valores possuidos.
```
```solution
let numeros = vec![1, 2, 3];
let nomes = vec![String::from("Ana")];
let valores: Vec<i32> = numeros.iter().copied().collect();
let nomes_clone: Vec<String> = nomes.iter().cloned().collect();
```
### Hints
- Use copied para i32 e cloned para String.
### Concepts
- copied e um atalho para clonar tipos Copy.
### Pitfalls
- copied nao funciona para String.
### Docs
- [Iterator::copied](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied)
- [Iterator::cloned](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
### Rules
- required_ast|copied|HasMethodCall|copied|Usou copied.|Esperava copied.
- required_ast|cloned|HasMethodCall|cloned|Usou cloned.|Esperava cloned.
- required_ast|collect|MinMethodCallCount|collect|2|Coletou duas vezes.|Esperava dois collect.

## into-iter-consume
module_id: iterators
title: Consumir colecao com into_iter
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado nomes: Vec<String>, crie maiusculas consumindo a colecao com into_iter, map e collect.
```
```scaffold
let nomes = vec![String::from("ana"), String::from("bia")];
```
```summary
into_iter toma posse dos itens, util quando voce quer transformar valores sem manter a colecao original.
```
```solution
let nomes = vec![String::from("ana"), String::from("bia")];
let maiusculas: Vec<String> = nomes.into_iter().map(|nome| nome.to_uppercase()).collect();
```
### Hints
- Depois de into_iter, nomes nao deve ser usado.
### Concepts
- into_iter consome o Vec e move cada String.
### Pitfalls
- Use iter se quiser apenas emprestar.
### Docs
- [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
### Rules
- required_ast|into-iter|HasMethodCall|into_iter|Consumiu a colecao.|Esperava into_iter.
- required_ast|map|HasMethodCall|map|Transformou itens.|Esperava map.
- required_ast|collect|HasMethodCall|collect|Coletou resultado.|Esperava collect.
