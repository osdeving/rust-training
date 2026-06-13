id: error-handling
title: Tratamento de erros
description: Result, Option para Result, transformacoes, propagacao e erros customizados.

## result-match
module_id: error-handling
title: Tratar Result com match
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado resultado: Result<i32, &str>, use match para criar valor, retornando 0 em Err.
```
```scaffold
let resultado: Result<i32, &str> = Ok(7);
```
```summary
Antes do operador ?, e importante entender que Result representa sucesso ou erro explicitamente.
```
```solution
let resultado: Result<i32, &str> = Ok(7);
let valor = match resultado {
    Ok(n) => n,
    Err(_) => 0,
};
```
### Hints
- Trate Ok e Err.
### Concepts
- Result<T, E> tem variantes Ok(T) e Err(E).
### Pitfalls
- Ignorar Err costuma esconder falhas.
### Docs
- [std::result::Result](https://doc.rust-lang.org/std/result/enum.Result.html)
### Rules
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_pattern|ok|Ok\s*\(\s*n\s*\)|Tratou Ok.|Esperava Ok.
- required_pattern|err|Err\s*\(\s*_\s*\)|Tratou Err.|Esperava Err.

## result-unwrap-or-else
module_id: error-handling
title: Usar unwrap_or_else
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado resultado: Result<i32, &str>, crie valor com unwrap_or_else retornando 0.
```
```scaffold
let resultado: Result<i32, &str> = Err("falha");
```
```summary
unwrap_or_else calcula um fallback apenas quando existe erro.
```
```solution
let resultado: Result<i32, &str> = Err("falha");
let valor = resultado.unwrap_or_else(|_| 0);
```
### Hints
- A closure recebe o erro.
### Concepts
- Fallback lazy evita trabalho desnecessario.
### Pitfalls
- unwrap_or_else ainda ignora o erro se voce descartar o argumento.
### Docs
- [Result::unwrap_or_else](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else)
### Rules
- required_ast|unwrap-or-else|HasMethodCall|unwrap_or_else|Usou fallback lazy.|Esperava unwrap_or_else.
- required_pattern|closure|\|_\|\s*0|Criou fallback.|Esperava closure retornando 0.

## result-map
module_id: error-handling
title: Transformar Ok com map
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado resultado: Result<i32, &str>, crie dobrado usando map(|n| n * 2).
```
```scaffold
let resultado: Result<i32, &str> = Ok(4);
```
```summary
map transforma apenas o valor Ok, mantendo Err intacto.
```
```solution
let resultado: Result<i32, &str> = Ok(4);
let dobrado = resultado.map(|n| n * 2);
```
### Hints
- map recebe uma closure para o valor de sucesso.
### Concepts
- O tipo de erro nao muda.
### Pitfalls
- Para transformar erro, use map_err.
### Docs
- [Result::map](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)
### Rules
- required_ast|map|HasMethodCall|map|Transformou Ok.|Esperava map.
- required_pattern|dobro|n\s*\*\s*2|Dobrou valor.|Esperava multiplicar por 2.

## result-map-err
module_id: error-handling
title: Transformar erro com map_err
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado resultado: Result<i32, &str>, crie convertido usando map_err(|e| e.to_string()).
```
```scaffold
let resultado: Result<i32, &str> = Err("falha");
```
```summary
map_err transforma apenas o erro, mantendo Ok intacto.
```
```solution
let resultado: Result<i32, &str> = Err("falha");
let convertido: Result<i32, String> = resultado.map_err(|e| e.to_string());
```
### Hints
- A closure recebe o erro.
### Concepts
- map_err e comum para adaptar erros entre camadas.
### Pitfalls
- map nao altera Err.
### Docs
- [Result::map_err](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
### Rules
- required_ast|map-err|HasMethodCall|map_err|Transformou erro.|Esperava map_err.
- required_ast|to-string|HasMethodCall|to_string|Converteu erro.|Esperava to_string.

## option-ok-or
module_id: error-handling
title: Converter Option em Result com ok_or
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado nome: Option<&str>, crie resultado usando ok_or("nome ausente").
```
```scaffold
let nome = Some("Ana");
```
```summary
ok_or transforma ausencia em erro, conectando Option com Result.
```
```solution
let nome = Some("Ana");
let resultado: Result<&str, &str> = nome.ok_or("nome ausente");
```
### Hints
- O argumento de ok_or vira o erro de None.
### Concepts
- Option expressa ausencia; Result expressa falha com motivo.
### Pitfalls
- ok_or cria o erro sempre; ok_or_else cria sob demanda.
### Docs
- [Option::ok_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or)
### Rules
- required_ast|ok-or|HasMethodCall|ok_or|Converteu Option.|Esperava ok_or.
- required_pattern|result|Result\s*<\s*&\s*str\s*,\s*&\s*str\s*>|Anotou Result.|Esperava Result.

## parse-int-result
module_id: error-handling
title: Converter string para numero tratando erro
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie fn ler_numero(texto: &str) -> Result<i32, std::num::ParseIntError> usando texto.parse().
```
```summary
parse retorna Result quando a conversao pode falhar.
```
```solution
fn ler_numero(texto: &str) -> Result<i32, std::num::ParseIntError> {
    texto.parse()
}

let valor = ler_numero("42");
```
### Hints
- O retorno da funcao pode ser o proprio resultado de parse.
### Concepts
- Result permite deixar o chamador decidir como lidar com erro.
### Pitfalls
- unwrap aqui transformaria erro em panic.
### Docs
- [str::parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|result|Result\s*<\s*i32\s*,\s*std::num::ParseIntError\s*>|Retornou Result especifico.|Esperava Result com ParseIntError.
- required_ast|parse|HasMethodCall|parse|Usou parse.|Esperava parse.

## custom-error-enum
module_id: error-handling
title: Criar enum de erro simples
difficulty: Advanced
compile_mode: FullProgram
```prompt
Crie enum Erro { Vazio, Invalido } e fn validar(texto: &str) -> Result<(), Erro> usando if para vazio.
```
```summary
Enums sao uma forma idiomatica de modelar erros conhecidos do dominio.
```
```solution
enum Erro {
    Vazio,
    Invalido,
}

fn validar(texto: &str) -> Result<(), Erro> {
    if texto.is_empty() {
        Err(Erro::Vazio)
    } else {
        Ok(())
    }
}

fn main() {
    let resultado = validar("ok");
}
```
### Hints
- Result<(), Erro> indica sucesso sem valor extra.
### Concepts
- Erros customizados documentam falhas esperadas.
### Pitfalls
- O enum precisa cobrir os erros que sua API promete.
### Docs
- [The Rust Book: Recoverable errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
### Rules
- required_pattern|enum|enum\s+Erro\s*\{|Declarou enum de erro.|Esperava enum Erro.
- required_pattern|result|Result\s*<\s*\(\s*\)\s*,\s*Erro\s*>|Retornou Result customizado.|Esperava Result<(), Erro>.
- required_ast|if|HasIf|Validou condicao.|Esperava if.

## from-error
module_id: error-handling
title: Implementar From para converter erro
difficulty: Advanced
compile_mode: FullProgram
```prompt
Crie enum Erro { Parse(std::num::ParseIntError) } e implemente From<std::num::ParseIntError> for Erro.
```
```summary
Implementar From permite que ? converta automaticamente erros entre tipos.
```
```solution
enum Erro {
    Parse(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Erro {
    fn from(error: std::num::ParseIntError) -> Erro {
        Erro::Parse(error)
    }
}

fn ler(texto: &str) -> Result<i32, Erro> {
    let numero = texto.parse()?;
    Ok(numero)
}

fn main() {
    let valor = ler("42");
}
```
### Hints
- A funcao obrigatoria se chama from.
### Concepts
- O operador ? usa From para converter o erro quando necessario.
### Pitfalls
- From deve ser conversao infalivel e sem surpresa.
### Docs
- [std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html)
### Rules
- required_pattern|impl-from|impl\s+From\s*<\s*std::num::ParseIntError\s*>\s+for\s+Erro|Implementou From.|Esperava impl From.
- required_pattern|from-fn|fn\s+from\s*\(\s*error\s*:\s*std::num::ParseIntError\s*\)\s*->\s*Erro|Criou from.|Esperava fn from.
- required_pattern|question|\?|Usou conversao com question.|Esperava ?.
- required_ast|parse|HasMethodCall|parse|Parseou texto.|Esperava parse.
