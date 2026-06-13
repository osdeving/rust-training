id: cli-input
title: CLI e entrada de usuario
description: stdin, read_line, parse de entrada e argumentos de linha de comando.

## stdin-read-line
module_id: cli-input
title: Ler linha do terminal
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie um main que le uma linha para String usando std::io::stdin().read_line(&mut linha)?.
```
```summary
read_line adiciona o texto lido a uma String mutavel e retorna Result.
```
```solution
fn main() -> std::io::Result<()> {
    let mut linha = String::new();
    std::io::stdin().read_line(&mut linha)?;
    Ok(())
}
```
### Hints
- A String precisa ser mutavel.
### Concepts
- read_line inclui a quebra de linha se ela existir.
### Pitfalls
- Nao esqueca de propagar ou tratar o erro.
### Docs
- [std::io::Stdin::read_line](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)
### Rules
- required_ast|string-new|HasCallPath|String::new|Criou buffer.|Esperava String::new.
- required_ast|read-line|HasMethodCall|read_line|Leu linha.|Esperava read_line.
- required_pattern|mut-ref|&\s*mut\s+linha|Passou buffer mutavel.|Esperava &mut linha.
- required_pattern|question|\?|Propagou erro.|Esperava ?.

## stdin-trim-parse
module_id: cli-input
title: Ler numero e converter
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie main que le uma linha, faz trim e parse para i32 propagando erro com Box<dyn std::error::Error>.
```
```summary
Para programas pequenos, Box<dyn Error> simplifica retorno de erros diferentes.
```
```solution
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut linha = String::new();
    std::io::stdin().read_line(&mut linha)?;
    let numero: i32 = linha.trim().parse()?;
    Ok(())
}
```
### Hints
- read_line e parse produzem erros diferentes.
### Concepts
- Box<dyn Error> usa trait object para unificar erros.
### Pitfalls
- trim remove a quebra de linha antes do parse.
### Docs
- [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)
### Rules
- required_ast|read-line|HasMethodCall|read_line|Leu entrada.|Esperava read_line.
- required_ast|trim|HasMethodCall|trim|Removeu quebra de linha.|Esperava trim.
- required_ast|parse|HasMethodCall|parse|Converteu numero.|Esperava parse.
- required_pattern|box-error|Box\s*<\s*dyn\s+std::error::Error\s*>|Retornou erro generico.|Esperava Box<dyn Error>.

## args-collect
module_id: cli-input
title: Ler argumentos com std::env::args
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie main que coleta std::env::args() em Vec<String>.
```
```summary
std::env::args retorna um iterador com os argumentos do processo.
```
```solution
fn main() {
    let args: Vec<String> = std::env::args().collect();
}
```
### Hints
- args e iterador; use collect.
### Concepts
- O primeiro argumento normalmente e o caminho do programa.
### Pitfalls
- Argumentos podem nao ser Unicode valido; args_os cobre esse caso.
### Docs
- [std::env::args](https://doc.rust-lang.org/std/env/fn.args.html)
### Rules
- required_pattern|args|std::env::args\s*\(\s*\)|Leu argumentos.|Esperava std::env::args().
- required_ast|collect|HasMethodCall|collect|Coletou args.|Esperava collect.
- required_ast|tipo|HasLetType|Vec|Anotou Vec.|Esperava Vec<String>.

## args-match-command
module_id: cli-input
title: Interpretar comando simples
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie main que pega o primeiro argumento com nth(1) e usa match para tratar Some("list") e fallback.
```
```summary
Comandos simples podem ser tratados com match em Option.
```
```solution
fn main() {
    let comando = std::env::args().nth(1);
    let acao = match comando.as_deref() {
        Some("list") => "listar",
        Some(_) => "desconhecido",
        None => "ajuda",
    };
}
```
### Hints
- as_deref converte Option<String> para Option<&str>.
### Concepts
- match deixa todos os casos de comando explicitos.
### Pitfalls
- nth(1) pula o nome do programa.
### Docs
- [Iterator::nth](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth)
- [Option::as_deref](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref)
### Rules
- required_pattern|args|std::env::args\s*\(\s*\)|Leu argumentos.|Esperava std::env::args().
- required_ast|nth|HasMethodCall|nth|Pegou argumento.|Esperava nth.
- required_ast|match|HasMatch|Usou match.|Esperava match.
- required_ast|as-deref|HasMethodCall|as_deref|Converteu para &str.|Esperava as_deref.
