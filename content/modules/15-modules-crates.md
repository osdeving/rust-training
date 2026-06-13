id: modules-crates
title: Modulos, use e pub
description: Organizacao em modulos, caminhos, visibilidade e APIs publicas em um arquivo.

## mod-inline
module_id: modules-crates
title: Criar modulo inline
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie um modulo inline calculos com pub fn dobro(n: i32) -> i32 e chame em main.
```
```summary
mod cria um namespace. Um modulo inline usa chaves no mesmo arquivo.
```
```solution
mod calculos {
    pub fn dobro(n: i32) -> i32 {
        n * 2
    }
}

fn main() {
    let valor = calculos::dobro(4);
}
```
### Hints
- A funcao precisa ser pub para ser chamada fora do modulo.
### Concepts
- Caminhos usam ::.
### Pitfalls
- Itens de modulo sao privados por padrao.
### Docs
- [The Rust Book: Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
### Rules
- required_pattern|mod|mod\s+calculos\s*\{|Criou modulo.|Esperava mod calculos.
- required_pattern|pub-fn|pub\s+fn\s+dobro|Expos funcao.|Esperava pub fn.
- required_pattern|path|calculos::dobro\s*\(|Chamou por caminho.|Esperava calculos::dobro.

## mod-file
module_id: modules-crates
title: Simular modulo de arquivo separado
difficulty: Beginner
compile_mode: FullProgram
```prompt
Em um unico arquivo, crie mod saudacoes com pub fn ola() -> &'static str e chame saudacoes::ola().
```
```summary
No projeto real, `mod saudacoes;` pode apontar para outro arquivo. Aqui usamos modulo inline para praticar o caminho.
```
```solution
mod saudacoes {
    pub fn ola() -> &'static str {
        "ola"
    }
}

fn main() {
    let texto = saudacoes::ola();
}
```
### Hints
- Use mod inline porque o app valida um arquivo por vez.
### Concepts
- O mesmo sistema de caminhos vale para modulos em arquivos separados.
### Pitfalls
- `mod saudacoes;` exigiria um arquivo real saudacoes.rs.
### Docs
- [The Rust Book: Separating modules into files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
### Rules
- required_pattern|mod|mod\s+saudacoes\s*\{|Criou modulo.|Esperava mod saudacoes.
- required_pattern|pub-fn|pub\s+fn\s+ola|Expos funcao.|Esperava pub fn ola.
- required_pattern|path|saudacoes::ola\s*\(|Chamou funcao do modulo.|Esperava chamada por caminho.

## use-path
module_id: modules-crates
title: Importar item com use
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie mod calculos com pub fn triplo, importe com use calculos::triplo e chame triplo(3).
```
```summary
use traz um caminho para o escopo atual, reduzindo repeticao.
```
```solution
mod calculos {
    pub fn triplo(n: i32) -> i32 {
        n * 3
    }
}

use calculos::triplo;

fn main() {
    let valor = triplo(3);
}
```
### Hints
- O use fica no nivel do arquivo.
### Concepts
- use nao torna item publico; so importa no escopo.
### Pitfalls
- O item importado ainda precisa estar visivel.
### Docs
- [The Rust Book: use keyword](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
### Rules
- required_pattern|use|use\s+calculos::triplo\s*;|Importou caminho.|Esperava use calculos::triplo.
- required_pattern|chamada|let\s+valor\s*=\s*triplo\s*\(3\)|Chamou item importado.|Esperava triplo(3).

## pub-function
module_id: modules-crates
title: Expor funcao publica
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie mod api com pub fn status() -> &'static str retornando "ok".
```
```summary
pub controla o que pode ser acessado de fora do modulo.
```
```solution
mod api {
    pub fn status() -> &'static str {
        "ok"
    }
}

fn main() {
    let status = api::status();
}
```
### Hints
- Sem pub, main nao consegue chamar api::status.
### Concepts
- Privacidade em Rust e por modulo.
### Pitfalls
- pub no modulo e pub na funcao sao coisas diferentes.
### Docs
- [The Rust Book: Privacy](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
### Rules
- required_pattern|pub-fn|pub\s+fn\s+status|Declarou funcao publica.|Esperava pub fn status.
- required_pattern|path|api::status\s*\(|Chamou API publica.|Esperava api::status.

## pub-struct-fields
module_id: modules-crates
title: Struct publica com campos privados
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie mod dominio com pub struct Usuario { nome: String } e pub fn novo(nome: String) -> Usuario.
```
```summary
Uma struct pode ser publica enquanto seus campos continuam privados.
```
```solution
mod dominio {
    pub struct Usuario {
        nome: String,
    }

    pub fn novo(nome: String) -> Usuario {
        Usuario { nome }
    }
}

fn main() {
    let usuario = dominio::novo(String::from("Ana"));
}
```
### Hints
- Pub na struct nao torna os campos publicos.
### Concepts
- Campos privados preservam invariantes.
### Pitfalls
- Fora do modulo, nao da para construir Usuario diretamente se campos sao privados.
### Docs
- [The Rust Book: Public structs and fields](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword)
### Rules
- required_pattern|pub-struct|pub\s+struct\s+Usuario|Declarou struct publica.|Esperava pub struct.
- required_pattern|private-field|nome\s*:\s*String|Manteve campo interno.|Esperava campo nome.
- required_pattern|constructor|pub\s+fn\s+novo|Criou construtor publico.|Esperava pub fn novo.

## impl-public-api
module_id: modules-crates
title: Criar API publica simples
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie mod contador com pub struct Contador, pub fn new() e pub fn valor(&self) -> i32.
```
```summary
Uma API publica pequena pode esconder campos e expor metodos seguros.
```
```solution
mod contador {
    pub struct Contador {
        valor: i32,
    }

    impl Contador {
        pub fn new() -> Contador {
            Contador { valor: 0 }
        }

        pub fn valor(&self) -> i32 {
            self.valor
        }
    }
}

fn main() {
    let contador = contador::Contador::new();
    let valor = contador.valor();
}
```
### Hints
- O campo pode ficar privado; os metodos publicos controlam acesso.
### Concepts
- `impl` tambem respeita visibilidade por item.
### Pitfalls
- Se new nao for pub, o chamador externo nao cria o valor.
### Docs
- [The Rust Book: Method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
### Rules
- required_pattern|pub-struct|pub\s+struct\s+Contador|Declarou tipo publico.|Esperava pub struct.
- required_pattern|new|pub\s+fn\s+new\s*\(\s*\)\s*->\s*Contador|Criou new publico.|Esperava pub fn new.
- required_pattern|valor|pub\s+fn\s+valor\s*\(\s*&\s*self\s*\)\s*->\s*i32|Criou getter publico.|Esperava pub fn valor.
- required_pattern|chamada|Contador::new\s*\(|Chamou API publica.|Esperava Contador::new.
