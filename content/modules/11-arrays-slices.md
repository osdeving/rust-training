id: arrays-slices
title: Arrays e slices
description: Arrays de tamanho fixo, slices, string slices e APIs que recebem fatias.

## array-fixed-size
module_id: arrays-slices
title: Criar array de tamanho fixo
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie numeros: [i32; 3] com os valores 1, 2 e 3.
```
```summary
Arrays tem tamanho fixo conhecido em tempo de compilacao.
```
```solution
let numeros: [i32; 3] = [1, 2, 3];
```
### Hints
- O tipo do array inclui tipo do item e tamanho.
### Concepts
- [i32; 3] nao e o mesmo tipo que [i32; 4].
### Pitfalls
- Array nao cresce como Vec.
### Docs
- [The Rust Book: Arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
### Rules
- required_pattern|array-type|:\s*\[\s*i32\s*;\s*3\s*\]|Declarou array fixo.|Esperava [i32; 3].
- required_pattern|valores|\[\s*1\s*,\s*2\s*,\s*3\s*\]|Criou valores.|Esperava array com 1, 2, 3.

## array-repeated-values
module_id: arrays-slices
title: Criar array com valores repetidos
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie zeros com cinco valores 0 usando a sintaxe [valor; n].
```
```summary
A forma [valor; n] cria um array repetindo o mesmo valor.
```
```solution
let zeros = [0; 5];
```
### Hints
- O ponto e virgula separa valor e quantidade.
### Concepts
- Essa sintaxe evita repetir manualmente muitos elementos.
### Pitfalls
- Use virgula para listar elementos diferentes; ponto e virgula para repeticao.
### Docs
- [The Rust Book: Arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
### Rules
- required_pattern|repeticao|\[\s*0\s*;\s*5\s*\]|Criou array repetido.|Esperava [0; 5].

## slice-from-array
module_id: arrays-slices
title: Criar slice a partir de array
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie meio como slice dos indices 1..3.
```
```scaffold
let numeros = [10, 20, 30, 40];
```
```summary
Slice e uma visao emprestada de parte de uma colecao contigua.
```
```solution
let numeros = [10, 20, 30, 40];
let meio = &numeros[1..3];
```
### Hints
- Use & antes da indexacao por range.
### Concepts
- O slice nao possui os dados; ele referencia o array.
### Pitfalls
- O fim do range e exclusivo.
### Docs
- [The Rust Book: Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
### Rules
- required_pattern|slice|&\s*numeros\s*\[\s*1\s*\.\.\s*3\s*\]|Criou slice do array.|Esperava &numeros[1..3].

## fn-recebe-slice
module_id: arrays-slices
title: Funcao que recebe &[i32]
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie soma(xs: &[i32]) -> i32 usando xs.iter().sum().
```
```summary
Receber slice deixa a funcao aceitar arrays, Vec e outras fatias.
```
```solution
fn soma(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

let total = soma(&[1, 2, 3]);
```
### Hints
- A assinatura deve receber &[i32].
### Concepts
- Slices sao uma interface comum para colecoes contiguas.
### Pitfalls
- &Vec<i32> e menos flexivel que &[i32].
### Docs
- [The Rust Book: Slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|parametro|xs\s*:\s*&\s*\[\s*i32\s*\]|Recebeu slice.|Esperava &[i32].
- required_ast|iter|HasMethodCall|iter|Iterou slice.|Esperava iter.
- required_ast|sum|HasMethodCall|sum|Somou valores.|Esperava sum.

## mutable-slice
module_id: arrays-slices
title: Alterar parte de array via &mut [i32]
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie array mutavel e um slice mutavel dos dois primeiros itens, depois altere o primeiro item do slice.
```
```summary
Um slice mutavel permite alterar uma janela de uma colecao sem expor tudo.
```
```solution
let mut numeros = [1, 2, 3];
let parte = &mut numeros[0..2];
parte[0] = 10;
```
### Hints
- A colecao original precisa ser mutavel.
### Concepts
- &mut [T] e um emprestimo exclusivo de uma fatia.
### Pitfalls
- Enquanto o slice mutavel existe, nao use o array original.
### Docs
- [std::primitive::slice](https://doc.rust-lang.org/std/primitive.slice.html)
### Rules
- required_ast|mut|HasLetMut|Criou array mutavel.|Esperava let mut.
- required_pattern|slice-mut|&\s*mut\s+numeros\s*\[\s*0\s*\.\.\s*2\s*\]|Criou slice mutavel.|Esperava &mut numeros[0..2].
- required_pattern|altera|parte\s*\[\s*0\s*\]\s*=|Alterou item pelo slice.|Esperava alterar parte[0].

## vec-as-slice
module_id: arrays-slices
title: Passar Vec para funcao que recebe slice
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie media(xs: &[i32]) -> i32 e chame com &numeros, onde numeros e Vec<i32>.
```
```summary
Vec pode ser emprestado como slice, o que deixa a API mais flexivel.
```
```solution
fn media(xs: &[i32]) -> i32 {
    xs.iter().sum::<i32>() / xs.len() as i32
}

let numeros = vec![2, 4, 6];
let valor = media(&numeros);
```
### Hints
- A funcao recebe slice; a chamada pode passar &Vec.
### Concepts
- Deref coercion permite converter &Vec<T> para &[T].
### Pitfalls
- Receber Vec por valor consumiria a colecao.
### Docs
- [The Rust Book: Deref coercions](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods)
### Rules
- required_pattern|parametro|xs\s*:\s*&\s*\[\s*i32\s*\]|Recebeu slice.|Esperava &[i32].
- required_ast|vec|HasMacroCall|vec|Criou Vec.|Esperava vec!.
- required_pattern|chamada|media\s*\(\s*&\s*numeros\s*\)|Chamou com referencia.|Esperava media(&numeros).

## string-slice-range
module_id: arrays-slices
title: Criar &str com range
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado texto = "Ferris", crie inicio como &texto[0..3].
```
```summary
String slices referenciam parte de um texto UTF-8 por intervalo de bytes valido.
```
```solution
let texto = "Ferris";
let inicio = &texto[0..3];
```
### Hints
- Use & antes do range.
### Concepts
- Ranges em string usam indices de byte.
### Pitfalls
- Cortar no meio de caractere UTF-8 causa panic.
### Docs
- [The Rust Book: String slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices)
### Rules
- required_pattern|string-slice|&\s*texto\s*\[\s*0\s*\.\.\s*3\s*\]|Criou string slice.|Esperava &texto[0..3].

## split-at
module_id: arrays-slices
title: Dividir slice com split_at
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numeros, use split_at(2) para criar duas fatias.
```
```scaffold
let numeros = [1, 2, 3, 4];
```
```summary
split_at divide um slice em duas fatias sem copiar os dados.
```
```solution
let numeros = [1, 2, 3, 4];
let (esquerda, direita) = numeros.split_at(2);
```
### Hints
- O retorno e uma tupla com duas fatias.
### Concepts
- A divisao preserva referencias para os mesmos dados.
### Pitfalls
- O indice precisa estar dentro do tamanho do slice.
### Docs
- [slice::split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)
### Rules
- required_ast|split-at|HasMethodCall|split_at|Dividiu o slice.|Esperava usar split_at.
- required_ast|tupla|HasTuplePatternBindingCount|2|Desestruturou retorno.|Esperava receber duas fatias.
