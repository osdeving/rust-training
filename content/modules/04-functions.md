id: functions
title: Funcoes
description: Parametros, retorno, referencias, tuplas e closures pequenas.

## fn-parametro-valor
module_id: functions
title: Criar funcao com parametro por valor
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma funcao dobro que recebe n: i32 e retorna n * 2.
```
```summary
Funcoes declaram tipos dos parametros e, quando retornam valor, o tipo de retorno.
```
```solution
fn dobro(n: i32) -> i32 {
    n * 2
}
let valor = dobro(6);
```
### Hints
- Retorne n * 2 como ultima expressao.
### Concepts
- A ultima expressao sem ; vira retorno.
### Pitfalls
- n * 2; retornaria ().
### Docs
- [The Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
### Rules
- required_ast|funcao|HasFunction|Declarou uma funcao.|Esperava uma funcao fn.
- required_ast|parametro|HasFunctionParamCount|1|Declarou parametro.|Esperava ao menos um parametro.
- required_ast|retorno|HasFunctionReturnType|Declarou tipo de retorno.|Esperava -> i32.
- required_pattern|nome|fn\s+dobro\s*\(|Nomeou a funcao como dobro.|Esperava fn dobro(...).

## fn-dois-parametros
module_id: functions
title: Somar com dois parametros
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie somar(a: i32, b: i32) -> i32 e use a funcao para calcular total.
```
```summary
Multiplos parametros sao separados por virgula, cada um com seu tipo.
```
```solution
fn somar(a: i32, b: i32) -> i32 {
    a + b
}
let total = somar(2, 3);
```
### Hints
- Cada parametro recebe seu proprio : i32.
### Concepts
- A chamada passa argumentos na ordem da assinatura.
### Pitfalls
- Nao escreva a, b: i32.
### Docs
- [The Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
### Rules
- required_ast|funcao|HasFunction|Declarou uma funcao.|Esperava uma funcao.
- required_ast|dois-parametros|HasFunctionParamCount|2|Declarou dois parametros.|Esperava dois parametros.
- required_ast|soma|HasBinaryAdd|Somou os parametros.|Esperava usar +.
- required_pattern|chamada|somar\s*\(|Chamou somar(...).|Esperava chamar a funcao.

## fn-parametro-referencia
module_id: functions
title: Receber String por referencia
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie tamanho(texto: &String) -> usize retornando texto.len() sem tomar posse da String.
```
```scaffold
let nome = String::from("Ferris");
```
```summary
Parametro por referencia permite ler um valor sem mover a posse para a funcao.
```
```solution
fn tamanho(texto: &String) -> usize {
    texto.len()
}
let nome = String::from("Ferris");
let total = tamanho(&nome);
```
### Hints
- Use &String na assinatura e &nome na chamada.
### Concepts
- Referencia empresta sem mover posse.
### Pitfalls
- &str costuma ser mais flexivel, mas aqui o foco e referencia.
### Docs
- [The Rust Book: References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_pattern|ref-param|texto\s*:\s*&\s*String|Recebeu &String.|Esperava parametro texto: &String.
- required_ast|len|HasMethodCall|len|Usou len().|Esperava texto.len().
- required_pattern|chamada-ref|tamanho\s*\(\s*&\s*nome\s*\)|Chamou com &nome.|Esperava chamar tamanho(&nome).

## fn-retorna-tupla
module_id: functions
title: Retornar tupla simples
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie min_max(a: i32, b: i32) -> (i32, i32) usando if/else para retornar menor e maior.
```
```summary
Tuplas agrupam poucos valores relacionados sem criar struct.
```
```solution
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a <= b { (a, b) } else { (b, a) }
}
let par = min_max(9, 4);
```
### Hints
- O retorno pode ser (a, b) ou (b, a).
### Concepts
- (i32, i32) e o tipo de uma tupla com dois inteiros.
### Pitfalls
- Os dois ramos precisam retornar o mesmo tipo.
### Docs
- [The Rust Book: Tuple type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_ast|if|HasIf|Usou if/else.|Esperava if/else.
- required_pattern|retorno-tupla|->\s*\(\s*i32\s*,\s*i32\s*\)|Declarou retorno de tupla.|Esperava -> (i32, i32).

## closure-map
module_id: functions
title: Usar closure em map
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numeros, crie quadrados usando .iter().map(|n| n * n).collect().
```
```scaffold
let numeros = vec![1, 2, 3];
```
```summary
Closures sao funcoes anonimas, uteis para transformar itens em iteradores.
```
```solution
let numeros = vec![1, 2, 3];
let quadrados: Vec<i32> = numeros.iter().map(|n| n * n).collect();
```
### Hints
- A closure fica dentro de map.
### Concepts
- |n| n * n e uma closure.
### Pitfalls
- map e lazy; use collect.
### Docs
- [The Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
### Rules
- required_ast|iter|HasMethodCall|iter|Usou iter().|Esperava .iter().
- required_ast|map|HasMethodCall|map|Usou map().|Esperava .map(...).
- required_ast|collect|HasMethodCall|collect|Usou collect().|Esperava .collect().
- required_pattern|closure|n\s*\*\s*n|Usou closure de quadrado.|Esperava multiplicar n por n.
