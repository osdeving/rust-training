id: files
title: Arquivos
description: Leitura, escrita, append e caminhos com a biblioteca padrao.

## fs-read-to-string
module_id: files
title: Ler arquivo como String
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie conteudo lendo notas.txt com std::fs::read_to_string e propague erro com ?.
```
```scaffold
fn main() -> std::io::Result<()> {
    use std::fs;
    
    Ok(())
}
```
```summary
read_to_string e o caminho curto para ler um arquivo textual inteiro.
```
```solution
fn main() -> std::io::Result<()> {
    use std::fs;
    let conteudo = fs::read_to_string("notas.txt")?;
    Ok(())
}
```
### Hints
- O operador ? propaga o erro.
### Concepts
- read_to_string retorna Result<String, io::Error>.
### Pitfalls
- Para arquivos grandes, pense em leitura incremental.
### Docs
- [std::fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
### Rules
- required_pattern|read|fs::read_to_string\s*\(|Usou fs::read_to_string.|Esperava fs::read_to_string(...).
- required_pattern|question|\?|Propagou erro com ?.|Esperava usar ?.

## fs-write
module_id: files
title: Escrever arquivo
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Escreva "feito" em saida.txt usando std::fs::write e propague erro com ?.
```
```scaffold
fn main() -> std::io::Result<()> {
    
    Ok(())
}
```
```summary
std::fs::write cria ou substitui um arquivo com todo o conteudo informado.
```
```solution
fn main() -> std::io::Result<()> {
    std::fs::write("saida.txt", "feito")?;
    Ok(())
}
```
### Hints
- Retorne Ok(()) no final.
### Concepts
- write recebe caminho e conteudo.
### Pitfalls
- write substitui o arquivo se ele existir.
### Docs
- [std::fs::write](https://doc.rust-lang.org/std/fs/fn.write.html)
### Rules
- required_pattern|write|(?:std::)?fs::write\s*\(|Usou fs::write.|Esperava std::fs::write(...).
- required_pattern|question|\?|Propagou erro com ?.|Esperava usar ?.

## openoptions-append
module_id: files
title: Adicionar ao fim com OpenOptions
difficulty: Advanced
compile_mode: FullProgram
```prompt
Abra log.txt para append criando o arquivo se necessario.
```
```scaffold
fn main() -> std::io::Result<()> {
    use std::fs::OpenOptions;
    
    Ok(())
}
```
```summary
OpenOptions configura como um arquivo deve ser aberto.
```
```solution
fn main() -> std::io::Result<()> {
    use std::fs::OpenOptions;
    let arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")?;
    Ok(())
}
```
### Hints
- Encadeie os metodos em OpenOptions::new().
### Concepts
- create(true) cria se nao existir.
- append(true) escreve no fim.
### Pitfalls
- Sem create(true), abrir arquivo inexistente falha.
### Docs
- [std::fs::OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)
### Rules
- required_pattern|openoptions|OpenOptions::new\s*\(|Usou OpenOptions::new().|Esperava OpenOptions::new().
- required_ast|create|HasMethodCall|create|Configurou create().|Esperava .create(true).
- required_ast|append|HasMethodCall|append|Configurou append().|Esperava .append(true).
- required_ast|open|HasMethodCall|open|Chamou open().|Esperava .open(...).

## path-exists
module_id: files
title: Checar se caminho existe
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie existe usando Path::new("Cargo.toml").exists().
```
```summary
Path representa caminhos de forma portavel.
```
```solution
use std::path::Path;
let existe = Path::new("Cargo.toml").exists();
```
### Hints
- exists() retorna bool.
### Concepts
- Path::new cria uma referencia de caminho.
### Pitfalls
- Entre a checagem e o uso, o arquivo ainda pode mudar.
### Docs
- [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html)
### Rules
- required_pattern|path|Path::new\s*\(|Usou Path::new().|Esperava Path::new(...).
- required_ast|exists|HasMethodCall|exists|Usou exists().|Esperava .exists().
