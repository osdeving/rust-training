id: pattern-matching
title: Pattern matching
description: Match com ranges, guards, if let, while let, matches! e desestruturacao.

## match-number-range
module_id: pattern-matching
title: Usar match com ranges
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado nota, crie conceito usando match com ranges 0..=59, 60..=89 e 90..=100.
```
```scaffold
let nota = 87;
```
```summary
Ranges em match deixam classificacoes numericas legiveis.
```
```solution
let nota = 87;
let conceito = match nota {
    0..=59 => "baixo",
    60..=89 => "medio",
    90..=100 => "alto",
    _ => "invalido",
};
```
### Hints
- Use _ para cobrir valores fora do intervalo.
### Concepts
- Match precisa ser exaustivo.
### Pitfalls
- 0..59 nao inclui 59; 0..=59 inclui.
### Docs
- [The Rust Book: Matching ranges](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#matching-ranges-of-values-with-)
### Rules
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_pattern|range|0\s*\.\.\s*=\s*59|Usou range inclusivo.|Esperava range inclusivo.
- required_pattern|fallback|_\s*=>|Tratou fallback.|Esperava _.

## match-guard
module_id: pattern-matching
title: Usar guarda com if no match
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado numero, use match com guarda if n % 2 == 0 para classificar par ou impar.
```
```scaffold
let numero = 8;
```
```summary
Match guards adicionam condicoes extras a um padrao.
```
```solution
let numero = 8;
let tipo = match numero {
    n if n % 2 == 0 => "par",
    _ => "impar",
};
```
### Hints
- O padrao pode capturar o valor em n.
### Concepts
- Guardas sao avaliadas depois do padrao bater.
### Pitfalls
- Guardas podem tornar cobertura menos obvia; mantenha fallback.
### Docs
- [The Rust Book: Match guards](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#extra-conditionals-with-match-guards)
### Rules
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_pattern|guard|if\s+n\s*%\s*2\s*==\s*0|Usou guarda.|Esperava if no match.
- required_pattern|fallback|_\s*=>|Tratou fallback.|Esperava _.

## if-let-option
module_id: pattern-matching
title: Tratar Some com if let
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado valor = Some(3), crie dobro com if let Some(n) = valor.
```
```summary
if let simplifica quando voce quer tratar um padrao principal e ignorar o resto.
```
```solution
let valor = Some(3);
let mut dobro = 0;
if let Some(n) = valor {
    dobro = n * 2;
}
```
### Hints
- Use if let para extrair n.
### Concepts
- if let e acucar sintatico para um match simples.
### Pitfalls
- Use match quando varios casos importam.
### Docs
- [The Rust Book: if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
### Rules
- required_pattern|if-let|if\s+let\s+Some\s*\(\s*n\s*\)\s*=|Usou if let Some.|Esperava if let.
- required_ast|mut|HasLetMut|Criou destino mutavel.|Esperava let mut.

## while-let-pop
module_id: pattern-matching
title: Consumir Vec com while let
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado pilha, some os valores removendo com while let Some(n) = pilha.pop().
```
```scaffold
let mut pilha = vec![1, 2, 3];
```
```summary
while let repete enquanto um padrao continuar batendo.
```
```solution
let mut pilha = vec![1, 2, 3];
let mut total = 0;
while let Some(n) = pilha.pop() {
    total += n;
}
```
### Hints
- pop retorna Option.
### Concepts
- while let combina loop e pattern matching.
### Pitfalls
- O Vec fica vazio ao final.
### Docs
- [The Rust Book: while let](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#conditional-while-let-loops)
### Rules
- required_pattern|while-let|while\s+let\s+Some\s*\(\s*n\s*\)\s*=|Usou while let.|Esperava while let Some.
- required_ast|pop|HasMethodCall|pop|Removeu itens.|Esperava pop.
- required_ast|mut|HasLetMut|Usou mutabilidade.|Esperava let mut.

## matches-macro
module_id: pattern-matching
title: Usar matches!
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado status = Some("ok"), crie ativo usando matches!(status, Some("ok")).
```
```summary
matches! retorna bool indicando se uma expressao bate com um padrao.
```
```solution
let status = Some("ok");
let ativo = matches!(status, Some("ok"));
```
### Hints
- matches! e macro.
### Concepts
- E util para checks pequenos em expressoes booleanas.
### Pitfalls
- Para extrair dados, prefira if let ou match.
### Docs
- [std::matches!](https://doc.rust-lang.org/std/macro.matches.html)
### Rules
- required_ast|matches|HasMacroCall|matches|Usou matches!.|Esperava matches!.
- required_pattern|some|Some\s*\(\s*"ok"\s*\)|Comparou o padrao.|Esperava Some("ok").

## destructure-struct
module_id: pattern-matching
title: Desestruturar struct em match
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare Ponto { x, y } e use match para classificar quando x e y forem zero.
```
```summary
Patterns podem abrir structs diretamente nos bracos do match.
```
```solution
struct Ponto {
    x: i32,
    y: i32,
}

let ponto = Ponto { x: 0, y: 0 };
let posicao = match ponto {
    Ponto { x: 0, y: 0 } => "origem",
    Ponto { x, y } => "outro",
};
```
### Hints
- Use o nome da struct no padrao.
### Concepts
- Padrões podem combinar campos literais e bindings.
### Pitfalls
- Se nao quiser usar campos, use `..`.
### Docs
- [The Rust Book: Destructuring structs](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#destructuring-structs)
### Rules
- required_pattern|struct|struct\s+Ponto\s*\{|Declarou Ponto.|Esperava struct.
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_pattern|pattern|Ponto\s*\{\s*x\s*:\s*0\s*,\s*y\s*:\s*0\s*\}|Usou padrao de struct.|Esperava padrao com campos.

## destructure-enum-data
module_id: pattern-matching
title: Extrair dados de enum
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare enum Mensagem { Texto(String), Sair } e use match para extrair o texto.
```
```summary
Variantes de enum podem carregar dados que sao extraidos por padrao.
```
```solution
enum Mensagem {
    Texto(String),
    Sair,
}

let msg = Mensagem::Texto(String::from("ola"));
let tamanho = match msg {
    Mensagem::Texto(texto) => texto.len(),
    Mensagem::Sair => 0,
};
```
### Hints
- O padrao da variante recebe um binding.
### Concepts
- O dado interno e movido para texto nesse match.
### Pitfalls
- Depois de mover a String, a enum original nao pode ser usada.
### Docs
- [The Rust Book: Enums and pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html)
### Rules
- required_pattern|enum|enum\s+Mensagem\s*\{|Declarou enum.|Esperava enum.
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_pattern|extrai|Mensagem::Texto\s*\(\s*texto\s*\)|Extraiu texto.|Esperava extrair dado da variante.
- required_ast|len|HasMethodCall|len|Usou dado extraido.|Esperava usar texto.

## ignore-values
module_id: pattern-matching
title: Ignorar valores com _ e ..
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare Config { host, porta, debug } e desestruture ignorando debug com ...
```
```summary
_ ignora um valor; .. ignora o restante dos campos.
```
```solution
struct Config {
    host: String,
    porta: u16,
    debug: bool,
}

let config = Config {
    host: String::from("localhost"),
    porta: 8080,
    debug: true,
};

let Config { host, porta, .. } = config;
```
### Hints
- Use .. dentro do padrao da struct.
### Concepts
- Ignorar campos deixa o padrao mais focado.
### Pitfalls
- Campos movidos ainda movem seus valores.
### Docs
- [The Rust Book: Ignoring values in a pattern](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern)
### Rules
- required_pattern|struct|struct\s+Config\s*\{|Declarou Config.|Esperava struct Config.
- required_pattern|ignore|Config\s*\{\s*host\s*,\s*porta\s*,\s*\.\.\s*\}|Ignorou campos restantes.|Esperava usar ...
