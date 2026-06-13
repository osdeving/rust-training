id: control-flow
title: Controle de fluxo
description: if, match e as principais formas de loop.

## if-else-paridade
module_id: control-flow
title: Classificar paridade com if/else
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma variavel que receba "par" se numero for par e "impar" caso contrario.
```
```scaffold
let numero = 8;
```
```summary
if em Rust e expressao: ele pode produzir um valor para uma variavel.
```
```solution
let numero = 8;
let rotulo = if numero % 2 == 0 { "par" } else { "impar" };
```
### Hints
- Use if condicao { valor } else { outro_valor }.
### Concepts
- Os dois ramos precisam produzir tipos compativeis.
### Pitfalls
- Nao coloque ; no valor final dos ramos.
### Docs
- [The Rust Book: if expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)
### Rules
- required_ast|if|HasIf|Usou if/else.|Esperava uma expressao if/else.
- required_pattern|modulo|%\s*2\s*==\s*0|Testou divisibilidade por 2.|Esperava testar numero % 2 == 0.
- required_ast|resultado|HasLetInitializerWithIf|Guardou o resultado do if.|Esperava uma variavel recebendo a expressao if.

## match-option
module_id: control-flow
title: Tratar Option com match
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Use match para criar texto: "valor presente" quando entrada for Some(_), e "sem valor" quando for None.
```
```scaffold
let entrada = Some(3);
```
```summary
match obriga voce a tratar todos os casos relevantes de um enum.
```
```solution
let entrada = Some(3);
let texto = match entrada {
    Some(_) => "valor presente",
    None => "sem valor",
};
```
### Hints
- Option tem os casos Some e None.
### Concepts
- Cada braco usa padrao => expressao.
### Pitfalls
- Nao esqueca o caso None.
### Docs
- [The Rust Book: match](https://doc.rust-lang.org/book/ch06-02-match.html)
### Rules
- required_ast|match|HasMatch|Usou match.|Esperava uma expressao match.
- required_pattern|some|Some\s*\(|Tratou Some(...).|Esperava um braco para Some(...).
- required_pattern|none|\bNone\b|Tratou None.|Esperava um braco para None.

## while-contador
module_id: control-flow
title: Contar regressivamente com while
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Comece com contador = 3 e use while para reduzir ate zero.
```
```summary
while repete enquanto a condicao continuar verdadeira.
```
```solution
let mut contador = 3;
while contador > 0 {
    contador -= 1;
}
```
### Hints
- Garanta que o corpo altere o contador.
### Concepts
- A condicao e reavaliada antes de cada volta.
### Pitfalls
- Sem decremento, o loop nunca termina.
### Docs
- [The Rust Book: loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops)
### Rules
- required_ast|while|HasWhileLoop|Usou while.|Esperava um loop while.
- required_ast|mut|HasLetMut|Usou variavel mutavel.|O contador precisa ser mutavel.
- required_pattern|decremento|contador\s*(?:-=|=\s*contador\s*-)|Reduziu o contador.|Esperava decrementar o contador.

## loop-break-valor
module_id: control-flow
title: Retornar valor com loop e break
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Use loop para incrementar n ate 4 e atribua o resultado de break n * 2 em dobro.
```
```scaffold
let mut n = 0;
```
```summary
loop pode retornar valor usando break expressao.
```
```solution
let mut n = 0;
let dobro = loop {
    n += 1;
    if n == 4 {
        break n * 2;
    }
};
```
### Hints
- A expressao loop pode ficar do lado direito de let dobro =.
### Concepts
- break valor vira o valor do loop.
### Pitfalls
- Sem break, loop nao termina.
### Docs
- [Returning values from loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)
### Rules
- required_ast|loop|HasLoop|Usou loop.|Esperava loop { ... }.
- required_ast|if|HasIf|Usou condicao de parada.|Esperava um if para decidir o break.
- required_pattern|break-valor|break\s+n\s*\*\s*2|Usou break com valor.|Esperava break n * 2.

## for-pares-tuplas
module_id: control-flow
title: Combinar itens com for aninhado
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado nomes e pontos, gere pares com tuplas (nome, ponto) usando dois loops for.
```
```scaffold
let nomes = vec!["Ana", "Bia"];
let pontos = vec![10, 20];
let mut pares = Vec::new();
```
```summary
Loops aninhados percorrem combinacoes: cada item externo combina com todos os internos.
```
```solution
let nomes = vec!["Ana", "Bia"];
let pontos = vec![10, 20];
let mut pares = Vec::new();
for nome in &nomes {
    for ponto in &pontos {
        pares.push((*nome, *ponto));
    }
}
```
### Hints
- Use for nome in &nomes e dentro dele for ponto in &pontos.
### Concepts
- Dois loops aninhados geram combinacoes.
### Pitfalls
- O custo cresce multiplicando os tamanhos.
### Docs
- [The Rust Book: for loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
### Rules
- required_ast|for|HasForLoop|Usou for.|Esperava loop for.
- required_ast|referencia|HasForLoopByReference|Iterou por referencia.|Esperava iterar com referencia.
- required_ast|push|HasMethodCall|push|Guardou combinacoes com push.|Esperava usar push em pares.
- min_pattern_count|dois-for|\bfor\b|2|Usou dois loops for.|Esperava dois loops for.
