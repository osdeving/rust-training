id: vectors
title: Vetores
description: Criacao, mutacao, acesso e iteracao com Vec<T>.

## vec-new-explicito
module_id: vectors
title: Criar Vec vazio com tipo explicito
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie um Vec vazio com tipo explicito.
```
```summary
Um Vec vazio precisa de uma pista de tipo. Aqui a pista vem da anotacao na variavel.
```
```solution
let numeros: Vec<i32> = Vec::new();
```
### Hints
- A anotacao de tipo fica na declaracao da variavel.
- O tipo esperado neste exercicio e Vec<i32>.
### Concepts
- Vec::new cria um vetor vazio.
- Sem valores iniciais, Rust precisa de uma pista de tipo.
### Pitfalls
- Vec<i32> nao e array [i32; N].
### Docs
- [The Rust Book: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
### Rules
- required_ast|vec-new|HasCallPath|Vec::new|Usou Vec::new().|Esperava encontrar Vec::new().
- required_ast|tipo-explicito|HasLetType|Vec|Declarou o tipo Vec<T> explicitamente.|Este exercicio pede uma anotacao explicita do tipo Vec<T>.

## vec-inferido-push
module_id: vectors
title: Inferir tipo no primeiro push
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie um Vec sem usar macro e com tipo inferido no primeiro push.
```
```summary
O vetor comeca vazio, mas o primeiro push da ao compilador a informacao de tipo.
```
```solution
let mut numeros = Vec::new();
numeros.push(10);
numeros.push(20);
```
### Hints
- O vetor precisa comecar vazio e depois receber valores.
- Para inserir depois da criacao, a variavel precisa ser mutavel.
### Concepts
- let mut permite alterar o vetor.
- push insere itens no fim do Vec.
### Pitfalls
- Sem mut, .push(...) nao compila.
- Nao use vec![...] quando o objetivo e praticar Vec::new.
### Docs
- [The Rust Book: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
### Rules
- required_ast|let-mut|HasLetMut|Declarou uma variavel mutavel.|Esperava uma declaracao com let mut.
- required_ast|vec-new|HasCallPath|Vec::new|Usou Vec::new().|Esperava encontrar Vec::new().
- required_ast|push-duas-vezes|MinMethodCallCount|push|2|Usou push pelo menos duas vezes.|Esperava pelo menos duas chamadas .push(...).
- forbidden_ast|sem-vec-macro|HasMacroCall|vec|Nao usou a macro vec!, como pedido.|Voce usou vec!, mas este exercicio queria Vec::new().
- forbidden_ast|sem-tipo-explicito|HasLetType|Vec|O tipo parece ter sido inferido.|Voce anotou Vec<T> explicitamente; este exercicio queria inferencia pelo push.

## vec-macro
module_id: vectors
title: Criar Vec com a macro vec!
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie um Vec com tres numeros em uma unica expressao.
```
```summary
A macro vec! e o atalho idiomatico para criar um vetor ja preenchido.
```
```solution
let numeros = vec![1, 2, 3];
```
### Hints
- Rust tem uma macro propria para literais de vetor.
### Concepts
- Macros usam ! antes dos delimitadores.
- vec![1, 2, 3] cria um Vec.
### Pitfalls
- Use colchetes: vec![...].
### Docs
- [std::vec! macro](https://doc.rust-lang.org/std/macro.vec.html)
### Rules
- required_ast|vec-macro|HasMacroCall|vec|Usou a macro vec!.|Esperava encontrar vec![...].
- forbidden_ast|sem-vec-new|HasCallPath|Vec::new|Nao usou Vec::new() neste exercicio.|Este exercicio pede vec!, nao Vec::new().

## vec-from-array
module_id: vectors
title: Criar Vec a partir de array
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Converta um array em Vec.
```
```summary
Arrays tem tamanho fixo; Vec cresce dinamicamente. Este exercicio pratica a conversao.
```
```solution
let numeros = Vec::from([1, 2, 3]);
```
### Hints
- Voce pode converter diretamente um array literal.
### Concepts
- Vec::from([...]) transforma array em vetor.
- [1, 2, 3].to_vec() tambem e comum.
### Pitfalls
- Array e Vec nao sao o mesmo tipo.
### Docs
- [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
### Rules
- required_ast|array-para-vec|HasArrayToVec|Usou uma conversao de array para Vec.|Esperava Vec::from([...]) ou .to_vec().
- forbidden_ast|sem-vec-macro|HasMacroCall|vec|Nao usou a macro vec!.|Este exercicio queria partir de um array, nao da macro vec!.

## vec-with-capacity
module_id: vectors
title: Reservar capacidade inicial
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um Vec ja reservando espaco para tres itens e depois preencha-o.
```
```summary
with_capacity reserva memoria, mas nao cria itens. O tamanho ainda comeca em zero.
```
```solution
let mut numeros = Vec::with_capacity(3);
numeros.push(1);
numeros.push(2);
numeros.push(3);
```
### Hints
- Capacidade reservada nao e a mesma coisa que tamanho.
### Concepts
- Capacidade e espaco reservado.
- Comprimento e quantidade de itens presentes.
### Pitfalls
- Nao acesse indices antes de inserir itens.
### Docs
- [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
### Rules
- required_ast|with-capacity|HasCallPathWithIntArg|Vec::with_capacity|3|Usou Vec::with_capacity(3).|Esperava encontrar Vec::with_capacity(3).
- required_ast|push-tres-vezes|MinMethodCallCount|push|3|Usou push pelo menos tres vezes.|Esperava pelo menos tres chamadas .push(...).

## vec-pop
module_id: vectors
title: Remover ultimo item com pop
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Remova o ultimo item de um Vec mutavel.
```
```summary
pop remove o ultimo item e devolve Option, porque o vetor pode estar vazio.
```
```solution
let mut numeros = vec![1, 2, 3];
let ultimo = numeros.pop();
```
### Hints
- A operacao altera o vetor.
### Concepts
- pop retorna Some(valor) ou None.
### Pitfalls
- Nao assuma que sempre existe ultimo item.
### Docs
- [std::vec::Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
### Rules
- required_ast|let-mut|HasLetMut|Declarou um Vec mutavel.|Esperava um Vec mutavel com let mut.
- required_ast|pop|HasMethodCall|pop|Usou pop().|Esperava encontrar uma chamada .pop().

## vec-for-referencia
module_id: vectors
title: Percorrer sem consumir o Vec
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Percorra um Vec sem consumi-lo.
```
```summary
Iterar com &numeros empresta o Vec. Assim ele continua disponivel depois do loop.
```
```solution
let numeros = vec![1, 2, 3];
for numero in &numeros {
    println!("{numero}");
}
```
### Hints
- O loop deve emprestar o vetor em vez de tomar posse dele.
### Concepts
- for numero in &numeros percorre referencias.
### Pitfalls
- for numero in numeros consome o vetor em muitos casos.
### Docs
- [The Rust Book: for loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
### Rules
- required_ast|for|HasForLoop|Usou um loop for.|Esperava um loop for.
- required_ast|referencia|HasForLoopByReference|Iterou por referencia.|Esperava iterar com &vetor para nao consumir o Vec.

## vec-map-collect
module_id: vectors
title: Transformar com map e collect
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um novo Vec com os valores dobrados a partir de outro Vec.
```
```summary
A cadeia iter().map(...).collect() transforma uma sequencia em outra colecao.
```
```solution
let numeros = vec![1, 2, 3];
let dobrados: Vec<i32> = numeros.iter().map(|n| n * 2).collect();
```
### Hints
- Use uma etapa de transformacao antes de materializar o novo Vec.
### Concepts
- iter cria um iterador por referencia.
- map transforma cada item.
- collect materializa o resultado.
### Pitfalls
- map sozinho e lazy.
### Docs
- [Iterator trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
### Rules
- required_ast|iter|HasMethodCall|iter|Usou iter().|Esperava encontrar .iter().
- required_ast|map|HasMethodCall|map|Usou map().|Esperava encontrar .map(...).
- required_ast|collect|HasMethodCall|collect|Usou collect().|Esperava encontrar .collect().
