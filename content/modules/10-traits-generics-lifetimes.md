id: traits-generics-lifetimes
title: Traits, generics e lifetimes
description: Comportamento compartilhado, bounds, tipos genericos, derive, Display e lifetimes.

## trait-define-impl
module_id: traits-generics-lifetimes
title: Definir trait e implementar
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare trait Descrever com fn descrever(&self) -> String. Implemente para Produto { nome: String }.
```
```summary
Traits descrevem comportamento compartilhado que tipos podem implementar.
```
```solution
trait Descrever {
    fn descrever(&self) -> String;
}

struct Produto {
    nome: String,
}

impl Descrever for Produto {
    fn descrever(&self) -> String {
        format!("Produto: {}", self.nome)
    }
}
```
### Hints
- O impl usa a forma impl Trait for Tipo.
### Concepts
- Traits separam a capacidade do tipo concreto.
### Pitfalls
- A assinatura no impl precisa bater com a trait.
### Docs
- [The Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
### Rules
- required_pattern|trait|trait\s+Descrever\s*\{|Declarou trait.|Esperava trait Descrever.
- required_pattern|assinatura|fn\s+descrever\s*\(\s*&\s*self\s*\)\s*->\s*String|Declarou metodo da trait.|Esperava assinatura descrever.
- required_pattern|impl-trait|impl\s+Descrever\s+for\s+Produto|Implementou trait para Produto.|Esperava impl Descrever for Produto.
- required_ast|format|HasMacroCall|format|Montou String formatada.|Esperava montar uma String.

## trait-default-method
module_id: traits-generics-lifetimes
title: Metodo default em trait
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Declare trait Nomeado com nome(&self) -> &str e metodo default etiqueta(&self) -> String usando format!.
```
```summary
Traits podem oferecer metodos com implementacao padrao em cima de metodos exigidos.
```
```solution
trait Nomeado {
    fn nome(&self) -> &str;

    fn etiqueta(&self) -> String {
        format!("Nome: {}", self.nome())
    }
}
```
### Hints
- O metodo default tem corpo dentro da trait.
### Concepts
- Uma implementacao default pode chamar outros metodos da mesma trait.
### Pitfalls
- Metodos sem corpo terminam com ponto e virgula.
### Docs
- [The Rust Book: Default implementations](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations)
### Rules
- required_pattern|trait|trait\s+Nomeado\s*\{|Declarou trait Nomeado.|Esperava trait Nomeado.
- required_pattern|nome|fn\s+nome\s*\(\s*&\s*self\s*\)\s*->\s*&\s*str\s*;|Declarou metodo exigido.|Esperava metodo nome sem corpo.
- required_pattern|etiqueta|fn\s+etiqueta\s*\(\s*&\s*self\s*\)\s*->\s*String\s*\{|Criou metodo default.|Esperava metodo etiqueta com corpo.
- required_ast|format|HasMacroCall|format|Usou formatacao.|Esperava formatar a etiqueta.

## trait-as-parameter
module_id: traits-generics-lifetimes
title: Receber parametro com trait bound
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie fn imprimir<T: std::fmt::Debug>(valor: T) que chama format!("{:?}", valor).
```
```summary
Trait bounds dizem quais comportamentos o tipo generico precisa oferecer.
```
```solution
fn imprimir<T: std::fmt::Debug>(valor: T) -> String {
    format!("{:?}", valor)
}

let texto = imprimir(42);
```
### Hints
- O bound fica dentro de <T: Trait>.
### Concepts
- Debug permite formatacao com {:?}.
### Pitfalls
- Sem bound, o compilador nao sabe se T pode ser formatado assim.
### Docs
- [The Rust Book: Trait bound syntax](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|bound|T\s*:\s*std::fmt::Debug|Usou bound Debug.|Esperava bound Debug.
- required_ast|format|HasMacroCall|format|Formatou valor.|Esperava usar format.

## impl-trait-return
module_id: traits-generics-lifetimes
title: Retornar impl Trait
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie fn ids() -> impl Iterator<Item = i32> retornando 1..=3.
```
```summary
impl Trait em retorno esconde o tipo concreto mantendo a capacidade prometida.
```
```solution
fn ids() -> impl Iterator<Item = i32> {
    1..=3
}

let total: i32 = ids().sum();
```
### Hints
- O retorno pode ser um range.
### Concepts
- O chamador conhece a trait, nao o tipo exato.
### Pitfalls
- Todos os caminhos de retorno precisam ter o mesmo tipo concreto.
### Docs
- [The Rust Book: impl Trait](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|impl-trait|->\s*impl\s+Iterator\s*<\s*Item\s*=\s*i32\s*>|Retornou impl Iterator.|Esperava impl Iterator.
- required_ast|sum|HasMethodCall|sum|Consumiu iterador.|Esperava usar o iterador retornado.

## generic-function
module_id: traits-generics-lifetimes
title: Funcao generica com trait bound
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie fn primeiro<T: Clone>(itens: &[T]) -> Option<T> retornando o primeiro item clonado.
```
```summary
Generics permitem escrever codigo para varios tipos, e bounds dizem quais capacidades sao necessarias.
```
```solution
fn primeiro<T: Clone>(itens: &[T]) -> Option<T> {
    itens.first().cloned()
}

let valor = primeiro(&[1, 2, 3]);
```
### Hints
- first retorna Option<&T>; cloned transforma em Option<T>.
### Concepts
- T: Clone e necessario para copiar o valor para fora da referencia.
### Pitfalls
- Sem Clone, voce nao pode produzir T a partir de &T genericamente.
### Docs
- [The Rust Book: Generic data types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_pattern|generic|fn\s+primeiro\s*<\s*T\s*:\s*Clone\s*>|Usou generic com bound.|Esperava T: Clone.
- required_pattern|slice|itens\s*:\s*&\s*\[\s*T\s*\]|Recebeu slice generico.|Esperava &[T].
- required_ast|first|HasMethodCall|first|Consultou primeiro item.|Esperava usar first.
- required_ast|cloned|HasMethodCall|cloned|Clonou o valor.|Esperava clonar o item.

## generic-struct
module_id: traits-generics-lifetimes
title: Struct generica
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare struct Par<T> { esquerda: T, direita: T } e crie Par<i32>.
```
```summary
Structs genericas reutilizam a mesma forma para diferentes tipos.
```
```solution
struct Par<T> {
    esquerda: T,
    direita: T,
}

let par = Par { esquerda: 1, direita: 2 };
```
### Hints
- O parametro generico aparece no nome da struct e nos campos.
### Concepts
- T representa um tipo escolhido no uso.
### Pitfalls
- Os dois campos usam o mesmo T, entao precisam ter tipos compativeis.
### Docs
- [The Rust Book: Generic structs](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)
### Rules
- required_pattern|struct-generica|struct\s+Par\s*<\s*T\s*>|Declarou struct generica.|Esperava Par<T>.
- required_pattern|campo-esquerda|esquerda\s*:\s*T|Campo usa T.|Esperava campo generico.
- required_pattern|instancia|Par\s*\{|Criou instancia.|Esperava instanciar Par.

## where-clause
module_id: traits-generics-lifetimes
title: Usar where clause
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie fn duplicar<T>(valor: T) -> (T, T) where T: Clone retornando (valor.clone(), valor).
```
```summary
where deixa bounds mais legiveis quando a assinatura cresce.
```
```solution
fn duplicar<T>(valor: T) -> (T, T)
where
    T: Clone,
{
    (valor.clone(), valor)
}

let par = duplicar(String::from("x"));
```
### Hints
- Coloque where antes do corpo da funcao.
### Concepts
- Bounds em where sao equivalentes aos bounds inline em muitos casos.
### Pitfalls
- O clone precisa acontecer antes de mover valor para a tupla.
### Docs
- [Rust Reference: Where clauses](https://doc.rust-lang.org/reference/items/generics.html#where-clauses)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_pattern|where|where\s+T\s*:\s*Clone|Usou where clause.|Esperava where T: Clone.
- required_ast|clone|HasMethodCall|clone|Clonou o valor.|Esperava usar clone.

## derive-debug-clone
module_id: traits-generics-lifetimes
title: Derivar traits comuns
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare struct Ponto { x: i32, y: i32 } com #[derive(Debug, Clone)] e clone uma instancia.
```
```summary
derive pede ao compilador para gerar implementacoes de traits comuns quando possivel.
```
```solution
#[derive(Debug, Clone)]
struct Ponto {
    x: i32,
    y: i32,
}

let a = Ponto { x: 1, y: 2 };
let b = a.clone();
```
### Hints
- O atributo fica acima da struct.
### Concepts
- Debug ajuda na inspecao; Clone permite duplicacao explicita.
### Pitfalls
- Clone nao e Copy; chamar clone deixa a intencao clara.
### Docs
- [Derive macro reference](https://doc.rust-lang.org/reference/attributes/derive.html)
### Rules
- required_pattern|derive|#\s*\[\s*derive\s*\(\s*Debug\s*,\s*Clone\s*\)\s*\]|Derivou Debug e Clone.|Esperava derive com traits comuns.
- required_pattern|struct|struct\s+Ponto\s*\{|Declarou Ponto.|Esperava struct Ponto.
- required_ast|clone|HasMethodCall|clone|Clonou uma instancia.|Esperava usar clone.

## display-impl
module_id: traits-generics-lifetimes
title: Implementar Display
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Declare Pessoa { nome: String } e implemente std::fmt::Display escrevendo o nome no Formatter.
```
```summary
Display controla a formatacao amigavel de um tipo com {}.
```
```solution
use std::fmt;

struct Pessoa {
    nome: String,
}

impl fmt::Display for Pessoa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nome)
    }
}
```
### Hints
- Display exige um metodo fmt com Formatter.
### Concepts
- write! escreve no formatter, nao necessariamente no terminal.
### Pitfalls
- O retorno precisa ser fmt::Result.
### Docs
- [std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
### Rules
- required_pattern|impl-display|impl\s+fmt::Display\s+for\s+Pessoa|Implementou Display.|Esperava impl Display para Pessoa.
- required_pattern|fmt|fn\s+fmt\s*\(\s*&\s*self\s*,\s*f\s*:\s*&\s*mut\s+fmt::Formatter|Declarou assinatura fmt.|Esperava metodo fmt correto.
- required_ast|write|HasMacroCall|write|Escreveu no formatter.|Esperava usar write.

## struct-lifetime-ref
module_id: traits-generics-lifetimes
title: Struct com referencia e lifetime
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Declare struct Trecho<'a> { texto: &'a str } e crie uma instancia usando uma string literal.
```
```summary
Structs que guardam referencias precisam declarar por quanto tempo essas referencias sao validas.
```
```solution
struct Trecho<'a> {
    texto: &'a str,
}

let trecho = Trecho { texto: "capitulo" };
```
### Hints
- O lifetime aparece no nome da struct e no campo de referencia.
### Concepts
- Lifetimes conectam a validade da struct a validade da referencia armazenada.
### Pitfalls
- Lifetime nao aumenta a vida do dado; so descreve a relacao.
### Docs
- [The Rust Book: Lifetime annotations in structs](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions)
### Rules
- required_pattern|lifetime-struct|struct\s+Trecho\s*<\s*'a\s*>\s*\{|Declarou lifetime na struct.|Esperava Trecho<'a>.
- required_pattern|campo|texto\s*:\s*&\s*'a\s*str|Guardou referencia com lifetime.|Esperava campo &'a str.
- required_pattern|instancia|Trecho\s*\{|Criou instancia.|Esperava instanciar Trecho.
