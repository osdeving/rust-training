id: basics
title: Variaveis e tipos basicos
description: let, mutabilidade, shadowing, anotacao de tipo, constantes e tipos escalares.

## let-imutavel
module_id: basics
title: Criar variavel imutavel
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma variavel resposta com o valor 42 sem usar mut.
```
```summary
Por padrao, variaveis em Rust sao imutaveis.
```
```solution
let resposta = 42;
```
### Hints
- Use let simples.
### Concepts
- Imutabilidade por padrao reduz mudancas acidentais.
### Pitfalls
- Nao use mut quando o valor nao muda.
### Docs
- [The Rust Book: Variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
### Rules
- required_pattern|let|let\s+resposta\s*=|Criou variavel resposta.|Esperava let resposta.
- forbidden_ast|sem-mut|HasLetMut|Nao usou mut.|Nao use mut neste exercicio.

## let-mut
module_id: basics
title: Alterar variavel mutavel
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie contador mutavel com 0 e depois some 1 nele.
```
```summary
mut libera reatribuicao ou alteracao de um binding.
```
```solution
let mut contador = 0;
contador += 1;
```
### Hints
- A palavra mut fica entre let e o nome.
### Concepts
- Mutabilidade precisa ser declarada explicitamente.
### Pitfalls
- Sem mut, a segunda linha nao compila.
### Docs
- [The Rust Book: Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
### Rules
- required_ast|mut|HasLetMut|Criou variavel mutavel.|Esperava let mut.
- required_pattern|incremento|contador\s*\+=\s*1|Alterou o contador.|Esperava incrementar contador.

## shadowing
module_id: basics
title: Usar shadowing
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie valor = "42" e depois use shadowing para transformar valor em i32 com parse.
```
```summary
Shadowing permite reutilizar o nome com outro valor e ate outro tipo.
```
```solution
let valor = "42";
let valor: i32 = valor.parse().unwrap_or(0);
```
### Hints
- Use let valor novamente.
### Concepts
- Shadowing nao e a mesma coisa que mutabilidade.
### Pitfalls
- Reatribuicao com = nao permite trocar o tipo.
### Docs
- [The Rust Book: Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
### Rules
- min_pattern_count|dois-let|let\s+valor|2|Usou shadowing.|Esperava declarar valor duas vezes.
- required_ast|parse|HasMethodCall|parse|Converteu texto.|Esperava usar parse.
- required_ast|tipo|HasLetType|i32|Indicou tipo i32.|Esperava anotacao i32.

## type-annotation
module_id: basics
title: Anotar tipo explicitamente
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie idade com tipo u8 e valor 30.
```
```summary
Anotacoes de tipo ajudam quando a inferencia nao e suficiente ou quando queremos deixar a intencao clara.
```
```solution
let idade: u8 = 30;
```
### Hints
- A anotacao fica depois do nome.
### Concepts
- O tipo faz parte do binding.
### Pitfalls
- u8 so aceita valores de 0 a 255.
### Docs
- [The Rust Book: Data types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
### Rules
- required_ast|tipo|HasLetType|u8|Anotou tipo u8.|Esperava anotacao u8.
- required_pattern|idade|let\s+idade\s*:\s*u8\s*=\s*30|Criou idade tipada.|Esperava idade: u8.

## const-basic
module_id: basics
title: Declarar constante
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Declare const LIMITE: usize = 10 e use esse valor em uma variavel.
```
```summary
Constantes precisam de tipo explicito e vivem por todo o escopo onde foram declaradas.
```
```solution
const LIMITE: usize = 10;
let tamanho = LIMITE;
```
### Hints
- Constantes usam const, nome e tipo.
### Concepts
- const nao e mutavel e precisa ser conhecida em tempo de compilacao.
### Pitfalls
- const exige anotacao de tipo.
### Docs
- [The Rust Book: Constants](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants)
### Rules
- required_pattern|const|const\s+LIMITE\s*:\s*usize\s*=\s*10|Declarou constante.|Esperava const LIMITE.
- required_pattern|uso|let\s+tamanho\s*=\s*LIMITE|Usou a constante.|Esperava usar LIMITE.

## numeric-types
module_id: basics
title: Combinar tipos numericos
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie a: i32 = 5, b: i32 = 7 e total com a + b.
```
```summary
Operacoes aritmeticas normalmente exigem tipos compativeis.
```
```solution
let a: i32 = 5;
let b: i32 = 7;
let total = a + b;
```
### Hints
- Use o mesmo tipo para os dois operandos.
### Concepts
- Rust nao converte todos os numeros automaticamente.
### Pitfalls
- Misturar i32 e usize pode exigir conversao explicita.
### Docs
- [The Rust Book: Integer types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)
### Rules
- min_pattern_count|dois-i32|:\s*i32|2|Anotou dois i32.|Esperava dois valores i32.
- required_ast|soma|HasBinaryAdd|Somou valores.|Esperava usar +.

## bool-char
module_id: basics
title: Usar bool e char
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie ativo: bool = true e inicial: char = 'R'.
```
```summary
bool e char sao tipos escalares basicos. char representa valor Unicode, nao byte.
```
```solution
let ativo: bool = true;
let inicial: char = 'R';
```
### Hints
- char usa aspas simples.
### Concepts
- bool tem true ou false.
### Pitfalls
- "R" e &str; 'R' e char.
### Docs
- [The Rust Book: Scalar types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)
### Rules
- required_ast|bool|HasLetType|bool|Criou bool.|Esperava tipo bool.
- required_ast|char|HasLetType|char|Criou char.|Esperava tipo char.

## basic-cast
module_id: basics
title: Converter numero com as
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie tamanho: usize = 3 e indice: i32 convertendo tamanho com as i32.
```
```summary
Conversoes numericas explicitas evitam surpresas e deixam a intencao visivel.
```
```solution
let tamanho: usize = 3;
let indice: i32 = tamanho as i32;
```
### Hints
- Use o operador as.
### Concepts
- Conversao numerica pode perder informacao se o destino for menor.
### Pitfalls
- Nao confunda cast numerico com parse de string.
### Docs
- [Rust Reference: Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
### Rules
- required_ast|usize|HasLetType|usize|Criou usize.|Esperava tipo usize.
- required_ast|i32|HasLetType|i32|Criou i32.|Esperava tipo i32.
- required_pattern|cast|as\s+i32|Converteu com as.|Esperava cast para i32.
