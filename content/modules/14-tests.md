id: tests
title: Testes
description: #[test], asserts, should_panic, testes com Result e mod tests.

## test-basic-assert
module_id: tests
title: Criar teste com assert!
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie uma funcao teste chamada soma_positiva usando #[test] e assert!(2 + 2 > 3).
```
```summary
Testes em Rust sao funcoes anotadas com #[test].
```
```solution
fn main() {}

#[test]
fn soma_positiva() {
    assert!(2 + 2 > 3);
}
```
### Hints
- A anotacao fica acima da funcao.
### Concepts
- assert! falha se a expressao booleana for false.
### Pitfalls
- Testes rodam com cargo test, nao com cargo run.
### Docs
- [The Rust Book: Writing tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
### Rules
- required_pattern|test-attr|#\s*\[\s*test\s*\]|Declarou teste.|Esperava #[test].
- required_ast|assert|HasMacroCall|assert|Usou assert!.|Esperava assert!.

## test-assert-eq
module_id: tests
title: Testar retorno com assert_eq!
difficulty: Beginner
compile_mode: FullProgram
```prompt
Crie fn dobro(n: i32) -> i32 e um teste usando assert_eq!(dobro(4), 8).
```
```summary
assert_eq! compara dois valores e mostra ambos quando falha.
```
```solution
fn dobro(n: i32) -> i32 {
    n * 2
}

fn main() {}

#[test]
fn dobra_numero() {
    assert_eq!(dobro(4), 8);
}
```
### Hints
- O valor esperado normalmente fica como segundo argumento.
### Concepts
- assert_eq! exige PartialEq e Debug.
### Pitfalls
- Mensagens de falha ficam melhores que assert!(a == b).
### Docs
- [std::assert_eq!](https://doc.rust-lang.org/std/macro.assert_eq.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|test-attr|#\s*\[\s*test\s*\]|Declarou teste.|Esperava #[test].
- required_ast|assert-eq|HasMacroCall|assert_eq|Usou assert_eq!.|Esperava assert_eq!.

## test-should-panic
module_id: tests
title: Testar panic esperado
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie um teste com #[should_panic] que chama panic!("falhou").
```
```summary
should_panic marca que o teste so passa se a funcao entrar em panic.
```
```solution
fn main() {}

#[test]
#[should_panic]
fn falha_esperada() {
    panic!("falhou");
}
```
### Hints
- Use duas anotacoes acima da funcao.
### Concepts
- Isso testa caminhos que devem rejeitar entrada invalida.
### Pitfalls
- should_panic muito amplo pode esconder panic errado.
### Docs
- [The Rust Book: should_panic](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic)
### Rules
- required_pattern|test-attr|#\s*\[\s*test\s*\]|Declarou teste.|Esperava #[test].
- required_pattern|should-panic|#\s*\[\s*should_panic\s*\]|Declarou should_panic.|Esperava #[should_panic].
- required_ast|panic|HasMacroCall|panic|Chamou panic!.|Esperava panic!.

## test-result-return
module_id: tests
title: Teste retornando Result
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie um teste parse_numero que retorna Result<(), std::num::ParseIntError> e usa ? ao parsear "42".
```
```summary
Testes podem retornar Result para usar ? em vez de unwrap.
```
```solution
fn main() {}

#[test]
fn parse_numero() -> Result<(), std::num::ParseIntError> {
    let numero: i32 = "42".parse()?;
    assert_eq!(numero, 42);
    Ok(())
}
```
### Hints
- Retorne Ok(()) no final.
### Concepts
- O operador ? propaga erro e faz o teste falhar.
### Pitfalls
- Nao misture should_panic com teste que retorna Result.
### Docs
- [The Rust Book: Using Result in tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests)
### Rules
- required_pattern|result-return|->\s*Result\s*<\s*\(\s*\)\s*,\s*std::num::ParseIntError\s*>|Retornou Result.|Esperava teste com Result.
- required_ast|parse|HasMethodCall|parse|Parseou texto.|Esperava parse.
- required_pattern|question|\?|Usou operador question.|Esperava ?.
- required_ast|assert-eq|HasMacroCall|assert_eq|Comparou resultado.|Esperava assert_eq.

## cfg-test-module
module_id: tests
title: Criar mod tests com #[cfg(test)]
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie fn somar e um modulo tests anotado com #[cfg(test)] contendo um teste.
```
```summary
O padrao comum e deixar testes unitarios em mod tests protegido por cfg(test).
```
```solution
fn somar(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma() {
        assert_eq!(somar(2, 3), 5);
    }
}
```
### Hints
- Dentro do modulo, use super::* para acessar itens externos.
### Concepts
- cfg(test) compila o modulo apenas durante testes.
### Pitfalls
- Sem use super::*, o teste pode nao enxergar funcoes do modulo pai.
### Docs
- [The Rust Book: Test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
### Rules
- required_pattern|cfg-test|#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]|Usou cfg(test).|Esperava #[cfg(test)].
- required_pattern|mod-tests|mod\s+tests\s*\{|Criou mod tests.|Esperava mod tests.
- required_pattern|use-super|use\s+super::\*|Importou itens do pai.|Esperava use super::*.
- required_ast|assert-eq|HasMacroCall|assert_eq|Usou assert_eq.|Esperava assert_eq.

## test-helper-function
module_id: tests
title: Usar funcao auxiliar em teste
difficulty: Intermediate
compile_mode: FullProgram
```prompt
Crie uma funcao auxiliar usuario_padrao dentro de mod tests e use em um teste com assert_eq!.
```
```summary
Funcoes auxiliares reduzem repeticao em testes sem precisarem de #[test].
```
```solution
struct Usuario {
    nome: String,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn usuario_padrao() -> Usuario {
        Usuario { nome: String::from("Ana") }
    }

    #[test]
    fn cria_usuario() {
        let usuario = usuario_padrao();
        assert_eq!(usuario.nome, "Ana");
    }
}
```
### Hints
- A helper nao recebe #[test].
### Concepts
- Testes sao codigo Rust normal.
### Pitfalls
- Helper sem uso pode gerar aviso, mas nao quebra.
### Docs
- [The Rust Book: Test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
### Rules
- required_pattern|helper|fn\s+usuario_padrao\s*\(\s*\)\s*->\s*Usuario|Criou helper.|Esperava funcao auxiliar.
- required_pattern|test-attr|#\s*\[\s*test\s*\]|Declarou teste.|Esperava #[test].
- required_ast|assert-eq|HasMacroCall|assert_eq|Validou resultado.|Esperava assert_eq.
