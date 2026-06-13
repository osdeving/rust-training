id: strings
title: Strings
description: Construcao e composicao basica com String.

## string-from-push-str
module_id: strings
title: Criar String e concatenar texto
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma String mutavel e acrescente outro trecho de texto.
```
```summary
String e texto alocado e mutavel. push_str adiciona uma fatia &str no fim.
```
```solution
let mut texto = String::from("Rust");
texto.push_str(" e legal");
```
### Hints
- A variavel precisa ser mutavel.
- Use um metodo proprio de String para acrescentar texto.
### Concepts
- String::from cria String a partir de &str.
- push_str altera o conteudo.
### Pitfalls
- Sem mut, push_str nao compila.
### Docs
- [The Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)
### Rules
- required_ast|string-from|HasCallPath|String::from|Usou String::from(...).|Esperava encontrar String::from(...).
- required_ast|let-mut|HasLetMut|Declarou a String como mutavel.|Esperava uma declaracao com let mut.
- required_ast|push-str|HasMethodCall|push_str|Usou push_str(...).|Esperava encontrar .push_str(...).

## string-format
module_id: strings
title: Compor texto com format!
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Componha uma String interpolando uma variavel.
```
```summary
format! monta uma nova String usando a mesma familia de formatacao do println!.
```
```solution
let linguagem = "Rust";
let mensagem = format!("Estou estudando {linguagem}");
```
### Hints
- Rust tem uma macro para montar String com interpolacao.
### Concepts
- format! retorna String em vez de imprimir.
### Pitfalls
- Concatenar com + pode mover a String da esquerda.
### Docs
- [std::format! macro](https://doc.rust-lang.org/std/macro.format.html)
- [The Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
### Rules
- required_ast|format|HasMacroCall|format|Usou format!.|Esperava encontrar format!(...).
- forbidden_ast|sem-concat-com-mais|HasBinaryAdd|Nao usou concatenacao com +.|Neste exercicio, prefira format! em vez de concatenar com +.

## string-new-push-char
module_id: strings
title: Construir String caractere por caractere
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma String vazia chamada palavra, adicione 'R' com push e depois acrescente "ust" com push_str.
```
```summary
String tem metodos diferentes para adicionar um char unico e uma fatia de texto.
```
```solution
let mut palavra = String::new();
palavra.push('R');
palavra.push_str("ust");
```
### Hints
- Use mut porque a String sera alterada.
- push recebe char; push_str recebe &str.
### Concepts
- String::new cria texto vazio alocado.
- push e push_str alteram a String existente.
### Pitfalls
- 'R' e char; "R" e &str.
### Docs
- [std::string::String::new](https://doc.rust-lang.org/std/string/struct.String.html#method.new)
- [std::string::String::push](https://doc.rust-lang.org/std/string/struct.String.html#method.push)
### Rules
- required_ast|string-new|HasCallPath|String::new|Criou uma String vazia.|Esperava criar String vazia.
- required_ast|mutavel|HasLetMut|Declarou valor mutavel.|Esperava let mut.
- required_ast|push-char|HasMethodCall|push|Usou metodo para adicionar caractere.|Esperava adicionar um caractere.
- required_ast|push-str|HasMethodCall|push_str|Usou metodo para adicionar texto.|Esperava acrescentar uma fatia de texto.

## string-to-string
module_id: strings
title: Converter literal para String
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Crie uma String chamada saudacao a partir do literal "ola" usando to_string().
```
```summary
to_string e uma forma comum de converter tipos que implementam Display em String.
```
```solution
let saudacao = "ola".to_string();
```
### Hints
- O metodo fica no valor que sera convertido.
### Concepts
- &str e uma fatia de string; String e texto alocado.
### Pitfalls
- to_string cria uma nova String; nao e so uma referencia.
### Docs
- [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html)
### Rules
- required_ast|to-string|HasMethodCall|to_string|Converteu usando to_string().|Esperava usar to_string().

## string-trim-parse
module_id: strings
title: Limpar texto e converter numero
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado entrada, remova espacos das bordas e converta para i32 usando parse.
```
```scaffold
let entrada = " 42 ";
```
```summary
trim remove espacos em volta do texto, e parse tenta converter para outro tipo.
```
```solution
let entrada = " 42 ";
let numero: i32 = entrada.trim().parse().unwrap_or(0);
```
### Hints
- Encadeie trim antes de parse.
- Diga ao compilador qual tipo parse deve produzir.
### Concepts
- parse retorna Result.
- A anotacao de tipo guia a conversao.
### Pitfalls
- parse sem tipo alvo costuma gerar erro de inferencia.
### Docs
- [str::trim](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
- [str::parse](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
### Rules
- required_ast|trim|HasMethodCall|trim|Removeu espacos das bordas.|Esperava limpar o texto.
- required_ast|parse|HasMethodCall|parse|Tentou converter o texto.|Esperava converter para numero.
- required_ast|tipo|HasLetType|i32|Indicou o tipo alvo.|Esperava deixar claro o tipo i32.

## string-split-whitespace
module_id: strings
title: Separar palavras ignorando espacos extras
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado frase, crie palavras: Vec<&str> usando split_whitespace e collect.
```
```scaffold
let frase = "rust  sem   medo";
```
```summary
split_whitespace quebra texto por qualquer sequencia de espacos Unicode.
```
```solution
let frase = "rust  sem   medo";
let palavras: Vec<&str> = frase.split_whitespace().collect();
```
### Hints
- O resultado e um iterador; use collect.
### Concepts
- Iteradores de string geralmente produzem fatias do texto original.
### Pitfalls
- split(" ") cria entradas vazias quando ha espacos repetidos.
### Docs
- [str::split_whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
### Rules
- required_ast|split-whitespace|HasMethodCall|split_whitespace|Separou por espacos.|Esperava separar palavras.
- required_ast|collect|HasMethodCall|collect|Coletou o iterador.|Esperava coletar em Vec.
- required_ast|tipo|HasLetType|Vec|Declarou colecao de palavras.|Esperava um Vec.

## string-replace
module_id: strings
title: Substituir trecho de texto
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado texto, crie corrigido substituindo "bug" por "fix".
```
```scaffold
let texto = "bug pequeno";
```
```summary
replace cria uma nova String com as ocorrencias substituidas.
```
```solution
let texto = "bug pequeno";
let corrigido = texto.replace("bug", "fix");
```
### Hints
- O metodo retorna uma nova String.
### Concepts
- &str nao precisa ser mutavel para gerar uma String substituida.
### Pitfalls
- replace troca todas as ocorrencias, nao apenas a primeira.
### Docs
- [str::replace](https://doc.rust-lang.org/std/primitive.str.html#method.replace)
### Rules
- required_ast|replace|HasMethodCall|replace|Usou substituicao de texto.|Esperava substituir parte do texto.

## string-contains-starts
module_id: strings
title: Consultar conteudo de uma string
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado nome, crie duas variaveis booleanas: tem_rs usando contains("rs") e comeca_com_c usando starts_with("c").
```
```scaffold
let nome = "crates";
```
```summary
Metodos de consulta retornam bool e ajudam a expressar intencao sem loops manuais.
```
```solution
let nome = "crates";
let tem_rs = nome.contains("rs");
let comeca_com_c = nome.starts_with("c");
```
### Hints
- Use um metodo para conter trecho e outro para prefixo.
### Concepts
- Consultas em str nao alteram o texto.
### Pitfalls
- starts_with verifica prefixo, nao busca em qualquer posicao.
### Docs
- [str::contains](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
- [str::starts_with](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
### Rules
- required_ast|contains|HasMethodCall|contains|Consultou trecho contido.|Esperava checar se contem texto.
- required_ast|starts-with|HasMethodCall|starts_with|Consultou prefixo.|Esperava checar prefixo.

## string-chars-count
module_id: strings
title: Contar caracteres Unicode
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado texto, crie total_chars contando caracteres com chars().count().
```
```scaffold
let texto = "olá";
```
```summary
len conta bytes; chars percorre valores Unicode escalares.
```
```solution
let texto = "olá";
let total_chars = texto.chars().count();
```
### Hints
- Use chars antes de count.
### Concepts
- Strings UTF-8 podem ter caracteres com mais de um byte.
### Pitfalls
- len em "olá" nao retorna a quantidade de letras visuais.
### Docs
- [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
### Rules
- required_ast|chars|HasMethodCall|chars|Iterou caracteres.|Esperava percorrer caracteres.
- required_ast|count|HasMethodCall|count|Contou os itens.|Esperava contar caracteres.

## string-bytes-len
module_id: strings
title: Comparar bytes e tamanho
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado texto, crie total_bytes usando len() e primeiro_byte usando bytes().next().
```
```scaffold
let texto = "Rust";
```
```summary
String em Rust e UTF-8; len trabalha em bytes.
```
```solution
let texto = "Rust";
let total_bytes = texto.len();
let primeiro_byte = texto.bytes().next();
```
### Hints
- bytes retorna iterador de u8.
### Concepts
- next em iteradores retorna Option.
### Pitfalls
- Indexar string por posicao numerica nao e permitido diretamente.
### Docs
- [str::bytes](https://doc.rust-lang.org/std/primitive.str.html#method.bytes)
- [str::len](https://doc.rust-lang.org/std/primitive.str.html#method.len)
### Rules
- required_ast|len|HasMethodCall|len|Consultou tamanho em bytes.|Esperava usar tamanho da string.
- required_ast|bytes|HasMethodCall|bytes|Percorreu bytes.|Esperava iterar bytes.
- required_ast|next|HasMethodCall|next|Pegou o primeiro item.|Esperava consultar o primeiro byte.

## string-join
module_id: strings
title: Juntar partes com separador
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Dado partes, crie caminho usando join("/") para montar "src/main.rs".
```
```scaffold
let partes = ["src", "main.rs"];
```
```summary
join combina uma colecao de fatias de string usando um separador.
```
```solution
let partes = ["src", "main.rs"];
let caminho = partes.join("/");
```
### Hints
- O separador vai como argumento de join.
### Concepts
- Arrays de &str conseguem usar join via trait de slice.
### Pitfalls
- format! e util, mas join escala melhor para varias partes.
### Docs
- [slice::join](https://doc.rust-lang.org/std/primitive.slice.html#method.join)
### Rules
- required_ast|join|HasMethodCall|join|Juntou partes com separador.|Esperava usar join.

## string-lines-filter
module_id: strings
title: Processar linhas nao vazias
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Dado texto, crie linhas uteis usando lines, filter para remover linhas vazias apos trim, e collect.
```
```scaffold
let texto = "a\n\n b ";
```
```summary
lines produz um iterador sobre linhas; filter permite manter apenas as relevantes.
```
```solution
let texto = "a\n\n b ";
let linhas_uteis: Vec<&str> = texto
    .lines()
    .filter(|linha| !linha.trim().is_empty())
    .collect();
```
### Hints
- Use trim dentro do filtro.
- Depois de filtrar, colete em Vec.
### Concepts
- filter recebe uma closure que decide se cada item fica.
### Pitfalls
- Uma linha com espacos nao e vazia ate voce aplicar trim.
### Docs
- [str::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
- [Iterator::filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
### Rules
- required_ast|lines|HasMethodCall|lines|Percorreu linhas.|Esperava usar linhas do texto.
- required_ast|filter|HasMethodCall|filter|Filtrou itens.|Esperava filtrar linhas.
- required_ast|trim|HasMethodCall|trim|Limpou espacos.|Esperava considerar espacos nas linhas.
- required_ast|collect|HasMethodCall|collect|Coletou resultado.|Esperava coletar as linhas.
