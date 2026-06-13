id: ownership-borrowing
title: Ownership e borrowing
description: Move, clone, Copy, emprestimos, escopos e mutabilidade por referencia.

## ownership-move-string
module_id: ownership-borrowing
title: Mover uma String
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie nome como String e mova sua posse para atual.
```
```summary
String nao implementa Copy. Atribuir uma String a outra variavel move a posse.
```
```solution
let nome = String::from("Ana");
let atual = nome;
```
### Hints
- Use String::from para criar um valor alocado.
- A atribuicao simples move a posse.
### Concepts
- Depois do move, a variavel original nao pode mais ser usada.
### Pitfalls
- Isso nao copia o buffer da String.
### Docs
- [The Rust Book: Move](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move)
### Rules
- required_ast|string-from|HasCallPath|String::from|Criou uma String.|Esperava criar uma String.
- required_pattern|move|let\s+atual\s*=\s*nome\s*;|Moveu a posse para outra variavel.|Esperava mover nome para atual.

## ownership-clone-string
module_id: ownership-borrowing
title: Clonar String mantendo a original
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie nome como String, clone para copia e ainda use nome depois.
```
```summary
clone faz uma copia profunda explicita quando voce precisa manter o valor original utilizavel.
```
```solution
let nome = String::from("Ana");
let copia = nome.clone();
let tamanho = nome.len();
```
### Hints
- Use clone antes de voltar a usar a variavel original.
### Concepts
- Clonar String copia o conteudo alocado.
### Pitfalls
- Clone tem custo; use referencia quando so precisa ler.
### Docs
- [The Rust Book: Clone](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone)
### Rules
- required_ast|string-from|HasCallPath|String::from|Criou uma String.|Esperava criar String.
- required_ast|clone|HasMethodCall|clone|Clonou explicitamente.|Esperava usar clone.
- required_ast|len|HasMethodCall|len|Usou a String original depois.|Esperava continuar usando o valor original.

## copy-i32
module_id: ownership-borrowing
title: Mostrar que i32 e Copy
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie a = 10, copie para b e depois calcule soma usando a e b.
```
```summary
Tipos simples como i32 implementam Copy. A atribuicao copia bits em vez de mover posse.
```
```solution
let a = 10;
let b = a;
let soma = a + b;
```
### Hints
- Inteiros continuam validos apos atribuicao.
### Concepts
- Copy e uma copia implicita barata para tipos simples.
### Pitfalls
- String nao se comporta assim.
### Docs
- [The Rust Book: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)
### Rules
- required_pattern|copy|let\s+b\s*=\s*a\s*;|Copiou o inteiro.|Esperava copiar a para b.
- required_ast|soma|HasBinaryAdd|Usou os dois valores.|Esperava combinar a e b.

## fn-toma-posse-string
module_id: ownership-borrowing
title: Funcao que toma posse de String
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie consumir(texto: String) que retorna texto.len(), e chame passando uma String.
```
```summary
Parametro por valor recebe a posse do argumento quando o tipo nao e Copy.
```
```solution
fn consumir(texto: String) -> usize {
    texto.len()
}

let nome = String::from("Ferris");
let tamanho = consumir(nome);
```
### Hints
- A assinatura deve receber String sem &.
### Concepts
- A chamada move nome para dentro da funcao.
### Pitfalls
- Depois da chamada, nome nao esta mais disponivel.
### Docs
- [The Rust Book: Ownership and functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_pattern|parametro|texto\s*:\s*String|Recebeu String por valor.|Esperava parametro String.
- required_ast|len|HasMethodCall|len|Usou o texto recebido.|Esperava consultar o tamanho.

## fn-empresta-string
module_id: ownership-borrowing
title: Funcao que empresta string
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie mostrar(texto: &str) -> usize retornando texto.len(), e chame com &nome.
```
```summary
Receber &str permite ler texto emprestado sem tomar posse de String.
```
```solution
fn mostrar(texto: &str) -> usize {
    texto.len()
}

let nome = String::from("Ferris");
let tamanho = mostrar(&nome);
```
### Hints
- &String pode virar &str por coerção.
### Concepts
- Emprestar evita mover o valor.
### Pitfalls
- &str e mais flexivel que &String para parametros de leitura.
### Docs
- [The Rust Book: References and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava uma funcao.
- required_pattern|parametro|texto\s*:\s*&\s*str|Recebeu &str.|Esperava parametro emprestado.
- required_pattern|chamada|mostrar\s*\(\s*&\s*nome\s*\)|Chamou emprestando nome.|Esperava chamar com referencia.

## borrow-len-sem-move
module_id: ownership-borrowing
title: Calcular tamanho sem mover
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie tamanho(texto: &String) -> usize, chame com &nome e use nome novamente.
```
```summary
Uma referencia imutavel permite ler sem tomar posse.
```
```solution
fn tamanho(texto: &String) -> usize {
    texto.len()
}

let nome = String::from("Rust");
let total = tamanho(&nome);
let depois = nome.len();
```
### Hints
- Use & na assinatura e na chamada.
### Concepts
- A posse continua com nome.
### Pitfalls
- Sem &, a funcao receberia a posse.
### Docs
- [The Rust Book: References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
### Rules
- required_pattern|parametro|texto\s*:\s*&\s*String|Recebeu referencia.|Esperava &String.
- required_pattern|chamada|tamanho\s*\(\s*&\s*nome\s*\)|Chamou por referencia.|Esperava emprestar nome.
- required_ast|len|MinMethodCallCount|len|2|Usou len mais de uma vez.|Esperava usar nome depois da chamada.

## mutable-borrow-change
module_id: ownership-borrowing
title: Alterar valor via &mut
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie adicionar_exclamacao(texto: &mut String) e use push('!') para alterar a String.
```
```summary
Referencia mutavel permite alterar um valor sem transferir a posse.
```
```solution
fn adicionar_exclamacao(texto: &mut String) {
    texto.push('!');
}

let mut mensagem = String::from("ok");
adicionar_exclamacao(&mut mensagem);
```
### Hints
- A variavel original e a referencia precisam ser mutaveis.
### Concepts
- &mut cria emprestimo exclusivo temporario.
### Pitfalls
- Nao pode haver outro emprestimo ativo do mesmo valor no mesmo momento.
### Docs
- [The Rust Book: Mutable references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
### Rules
- required_pattern|parametro-mut|texto\s*:\s*&\s*mut\s+String|Recebeu referencia mutavel.|Esperava &mut String.
- required_ast|let-mut|HasLetMut|Criou valor mutavel.|Esperava let mut.
- required_ast|push|HasMethodCall|push|Alterou a String.|Esperava modificar o texto.
- required_pattern|chamada-mut|&\s*mut\s+mensagem|Chamou com &mut.|Esperava emprestar mutavelmente.

## borrow-scope
module_id: ownership-borrowing
title: Encerrar emprestimo com escopo
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um Vec mutavel, leia o primeiro item dentro de um bloco e depois faca push no Vec.
```
```summary
Um emprestimo termina quando seu ultimo uso sai de escopo, liberando a mutacao depois.
```
```solution
let mut numeros = vec![1, 2, 3];
{
    let primeiro = &numeros[0];
    let valor = *primeiro;
}
numeros.push(4);
```
### Hints
- Use um bloco para limitar a vida da referencia.
### Concepts
- Escopos menores ajudam o borrow checker.
### Pitfalls
- Nao tente mutar o Vec enquanto a referencia ainda sera usada.
### Docs
- [The Rust Book: References scope](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
### Rules
- required_ast|let-mut|HasLetMut|Criou Vec mutavel.|Esperava let mut.
- required_ast|push|HasMethodCall|push|Mutou depois do emprestimo.|Esperava inserir depois.
- required_pattern|escopo|\{\s*let\s+primeiro\s*=\s*&\s*numeros\s*\[0\]|Usou escopo para referencia.|Esperava limitar o emprestimo.

## vec-borrow-then-push
module_id: ownership-borrowing
title: Emprestar e depois mutar Vec com segurança
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um Vec, copie o primeiro valor para uma variavel e depois faca push sem manter referencia ativa.
```
```summary
Copiar um valor Copy para fora evita manter uma referencia que bloquearia a mutacao do Vec.
```
```solution
let mut numeros = vec![1, 2, 3];
let primeiro = numeros[0];
numeros.push(4);
let soma = primeiro + numeros.len();
```
### Hints
- Para i32, ler por indice copia o valor.
### Concepts
- Referencias ativas restringem mutacao; valores Copy nao mantem emprestimo.
### Pitfalls
- let primeiro = &numeros[0]; seguido de push e uso da referencia nao compila.
### Docs
- [The Rust Book: Mutable references restrictions](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
### Rules
- required_ast|let-mut|HasLetMut|Criou Vec mutavel.|Esperava let mut.
- required_ast|push|HasMethodCall|push|Inseriu depois da leitura.|Esperava mutar o Vec.
- required_ast|soma|HasBinaryAdd|Usou o valor lido depois.|Esperava usar o primeiro valor.

## dangling-reference-fix
module_id: ownership-borrowing
title: Evitar referencia pendurada
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie criar_nome() -> String retornando uma String possuida, em vez de retornar referencia para valor local.
```
```summary
Rust impede referencias penduradas. Quando o dado e criado dentro da funcao, retorne o valor possuido.
```
```solution
fn criar_nome() -> String {
    String::from("Ferris")
}

let nome = criar_nome();
```
### Hints
- Retorne String, nao &str apontando para variavel local.
### Concepts
- O dono do valor retornado passa para quem chamou a funcao.
### Pitfalls
- &String ou &str para dado local morreria no fim da funcao.
### Docs
- [The Rust Book: Dangling references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)
### Rules
- required_ast|funcao|HasFunction|Declarou funcao.|Esperava funcao.
- required_ast|retorno|HasFunctionReturnType|Declarou retorno.|Esperava tipo de retorno.
- required_pattern|string-retorno|->\s*String|Retornou valor possuido.|Esperava retornar String.
- required_ast|string-from|HasCallPath|String::from|Criou String possuida.|Esperava criar String.
