id: options-results
title: Option e Result
description: Tratamento explicito de ausencia e erro.

## option-unwrap-or
module_id: options-results
title: Valor padrao com unwrap_or
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado apelido: Option<&str>, crie nome usando unwrap_or("anonimo").
```
```scaffold
let apelido: Option<&str> = None;
```
```summary
unwrap_or troca None por um valor padrao sem causar panic.
```
```solution
let apelido: Option<&str> = None;
let nome = apelido.unwrap_or("anonimo");
```
### Hints
- Use um fallback literal.
### Concepts
- Option representa presenca ou ausencia.
### Pitfalls
- Evite unwrap em fluxo normal de usuario.
### Docs
- [std::option::Option](https://doc.rust-lang.org/std/option/enum.Option.html)
### Rules
- required_ast|unwrap-or|HasMethodCall|unwrap_or|Usou unwrap_or().|Esperava .unwrap_or(...).
- forbidden_ast|sem-unwrap|HasMethodCall|unwrap|Nao usou unwrap().|Evite unwrap() neste exercicio.

## result-question
module_id: options-results
title: Propagar erro com ?
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie parse_id(texto: &str) -> Result<u32, std::num::ParseIntError> usando texto.parse()?.
```
```summary
? reduz boilerplate quando a funcao tambem retorna Result.
```
```solution
fn parse_id(texto: &str) -> Result<u32, std::num::ParseIntError> {
    let id = texto.parse()?;
    Ok(id)
}
let id = parse_id("42");
```
### Hints
- Use Ok(id) no final.
### Concepts
- ? retorna cedo se houver erro.
### Pitfalls
- ? so funciona em funcoes que retornam tipo compativel.
### Docs
- [The Rust Book: Result and ?](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_pattern|result|Result\s*<|Retorna Result.|Esperava retorno Result.
- required_pattern|question|\?|Usou ?.|Esperava usar ?.
- required_pattern|ok|Ok\s*\(|Retornou Ok(...).|Esperava Ok(...).

## if-let-some
module_id: options-results
title: Desestruturar Some com if let
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Use if let Some(...) = usuario para preencher saudacao quando houver nome.
```
```scaffold
let usuario = Some("Ana");
let mut saudacao = String::from("Oi");
```
```summary
if let e um atalho quando voce quer tratar um padrao especifico.
```
```solution
let usuario = Some("Ana");
let mut saudacao = String::from("Oi");
if let Some(nome) = usuario {
    saudacao = format!("Oi, {nome}");
}
```
### Hints
- Atualize saudacao dentro do bloco.
### Concepts
- if let roda o bloco apenas se houver valor.
### Pitfalls
- Use match se precisar tratar varios casos.
### Docs
- [The Rust Book: if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
### Rules
- required_ast|if-let|HasIfLetSome|Desestruturou Some.|Esperava if let Some(...).
- required_ast|format|HasMacroCall|format|Usou format!.|Esperava format!.
