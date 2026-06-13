# Catalogo de exercicios

> Arquivo gerado automaticamente a partir de `content/modules/*.md`. Nao edite manualmente; altere os modulos e recarregue o app.

Total: 19 modulo(s), 135 exercicio(s).

## Modulos

| # | Modulo | Arquivo | Exercicios | Iniciante | Intermediario | Avancado |
|---:|---|---|---:|---:|---:|---:|
| 1 | Variaveis e tipos basicos | `00-basics.md` | 8 | 7 | 1 | 0 |
| 2 | Vetores | `01-vectors.md` | 8 | 5 | 3 | 0 |
| 3 | Strings | `02-strings.md` | 12 | 6 | 5 | 1 |
| 4 | Controle de fluxo | `03-control-flow.md` | 5 | 3 | 2 | 0 |
| 5 | Funcoes | `04-functions.md` | 5 | 2 | 3 | 0 |
| 6 | Colecoes padrao | `05-collections.md` | 5 | 1 | 3 | 1 |
| 7 | Option e Result | `06-options-results.md` | 3 | 1 | 2 | 0 |
| 8 | Arquivos | `07-files.md` | 4 | 1 | 2 | 1 |
| 9 | Ownership e borrowing | `08-ownership-borrowing.md` | 10 | 6 | 3 | 1 |
| 10 | Tuplas, structs e enums | `09-tuples-structs-enums.md` | 10 | 5 | 5 | 0 |
| 11 | Traits, generics e lifetimes | `10-traits-generics-lifetimes.md` | 10 | 0 | 4 | 6 |
| 12 | Arrays e slices | `11-arrays-slices.md` | 8 | 3 | 5 | 0 |
| 13 | Iteradores | `12-iterators.md` | 10 | 3 | 6 | 1 |
| 14 | Pattern matching | `13-pattern-matching.md` | 8 | 3 | 5 | 0 |
| 15 | Testes | `14-tests.md` | 6 | 2 | 4 | 0 |
| 16 | Modulos, use e pub | `15-modules-crates.md` | 6 | 4 | 2 | 0 |
| 17 | Tratamento de erros | `16-error-handling.md` | 8 | 2 | 4 | 2 |
| 18 | CLI e entrada de usuario | `17-cli-input.md` | 4 | 1 | 3 | 0 |
| 19 | Smart pointers | `18-smart-pointers.md` | 5 | 0 | 1 | 4 |

## Exercicios

### Variaveis e tipos basicos (`00-basics.md`)

let, mutabilidade, shadowing, anotacao de tipo, constantes e tipos escalares.

- `let-imutavel` - Criar variavel imutavel (Iniciante)
- `let-mut` - Alterar variavel mutavel (Iniciante)
- `shadowing` - Usar shadowing (Iniciante)
- `type-annotation` - Anotar tipo explicitamente (Iniciante)
- `const-basic` - Declarar constante (Iniciante)
- `numeric-types` - Combinar tipos numericos (Iniciante)
- `bool-char` - Usar bool e char (Iniciante)
- `basic-cast` - Converter numero com as (Intermediario)

### Vetores (`01-vectors.md`)

Criacao, mutacao, acesso e iteracao com Vec<T>.

- `vec-new-explicito` - Criar Vec vazio com tipo explicito (Iniciante)
- `vec-inferido-push` - Inferir tipo no primeiro push (Iniciante)
- `vec-macro` - Criar Vec com a macro vec! (Iniciante)
- `vec-from-array` - Criar Vec a partir de array (Iniciante)
- `vec-with-capacity` - Reservar capacidade inicial (Intermediario)
- `vec-pop` - Remover ultimo item com pop (Iniciante)
- `vec-for-referencia` - Percorrer sem consumir o Vec (Intermediario)
- `vec-map-collect` - Transformar com map e collect (Intermediario)

### Strings (`02-strings.md`)

Construcao e composicao basica com String.

- `string-from-push-str` - Criar String e concatenar texto (Iniciante)
- `string-format` - Compor texto com format! (Iniciante)
- `string-new-push-char` - Construir String caractere por caractere (Iniciante)
- `string-to-string` - Converter literal para String (Iniciante)
- `string-trim-parse` - Limpar texto e converter numero (Intermediario)
- `string-split-whitespace` - Separar palavras ignorando espacos extras (Intermediario)
- `string-replace` - Substituir trecho de texto (Iniciante)
- `string-contains-starts` - Consultar conteudo de uma string (Iniciante)
- `string-chars-count` - Contar caracteres Unicode (Intermediario)
- `string-bytes-len` - Comparar bytes e tamanho (Intermediario)
- `string-join` - Juntar partes com separador (Intermediario)
- `string-lines-filter` - Processar linhas nao vazias (Avancado)

### Controle de fluxo (`03-control-flow.md`)

if, match e as principais formas de loop.

- `if-else-paridade` - Classificar paridade com if/else (Iniciante)
- `match-option` - Tratar Option com match (Iniciante)
- `while-contador` - Contar regressivamente com while (Iniciante)
- `loop-break-valor` - Retornar valor com loop e break (Intermediario)
- `for-pares-tuplas` - Combinar itens com for aninhado (Intermediario)

### Funcoes (`04-functions.md`)

Parametros, retorno, referencias, tuplas e closures pequenas.

- `fn-parametro-valor` - Criar funcao com parametro por valor (Iniciante)
- `fn-dois-parametros` - Somar com dois parametros (Iniciante)
- `fn-parametro-referencia` - Receber String por referencia (Intermediario)
- `fn-retorna-tupla` - Retornar tupla simples (Intermediario)
- `closure-map` - Usar closure em map (Intermediario)

### Colecoes padrao (`05-collections.md`)

HashMap, HashSet, VecDeque, stack com Vec e mapas ordenados.

- `hashmap-insert-get` - Contar pontos com HashMap (Intermediario)
- `hashset-unicos` - Guardar valores unicos com HashSet (Intermediario)
- `vecdeque-fila` - Fila com VecDeque (Intermediario)
- `vec-stack` - Pilha com Vec (Iniciante)
- `btreemap-ordenado` - Mapa ordenado com BTreeMap (Avancado)

### Option e Result (`06-options-results.md`)

Tratamento explicito de ausencia e erro.

- `option-unwrap-or` - Valor padrao com unwrap_or (Iniciante)
- `result-question` - Propagar erro com ? (Intermediario)
- `if-let-some` - Desestruturar Some com if let (Intermediario)

### Arquivos (`07-files.md`)

Leitura, escrita, append e caminhos com a biblioteca padrao.

- `fs-read-to-string` - Ler arquivo como String (Intermediario)
- `fs-write` - Escrever arquivo (Intermediario)
- `openoptions-append` - Adicionar ao fim com OpenOptions (Avancado)
- `path-exists` - Checar se caminho existe (Iniciante)

### Ownership e borrowing (`08-ownership-borrowing.md`)

Move, clone, Copy, emprestimos, escopos e mutabilidade por referencia.

- `ownership-move-string` - Mover uma String (Iniciante)
- `ownership-clone-string` - Clonar String mantendo a original (Iniciante)
- `copy-i32` - Mostrar que i32 e Copy (Iniciante)
- `fn-toma-posse-string` - Funcao que toma posse de String (Iniciante)
- `fn-empresta-string` - Funcao que empresta string (Iniciante)
- `borrow-len-sem-move` - Calcular tamanho sem mover (Iniciante)
- `mutable-borrow-change` - Alterar valor via &mut (Intermediario)
- `borrow-scope` - Encerrar emprestimo com escopo (Intermediario)
- `vec-borrow-then-push` - Emprestar e depois mutar Vec com segurança (Intermediario)
- `dangling-reference-fix` - Evitar referencia pendurada (Avancado)

### Tuplas, structs e enums (`09-tuples-structs-enums.md`)

Modelagem de dados com tuplas, structs, metodos, funcoes associadas e enums.

- `tuple-destructure` - Desestruturar tupla (Iniciante)
- `tuple-access` - Acessar campos de tupla por indice (Iniciante)
- `struct-named-fields` - Criar struct com campos nomeados (Iniciante)
- `struct-field-update` - Atualizar struct mutavel (Iniciante)
- `tuple-struct` - Criar tuple struct (Iniciante)
- `impl-method` - Implementar metodo em struct (Intermediario)
- `associated-function` - Criar funcao associada new (Intermediario)
- `enum-basic-match` - Criar enum e tratar com match (Intermediario)
- `enum-data-variant` - Enum com dados associados (Intermediario)
- `struct-destructure` - Desestruturar struct (Intermediario)

### Traits, generics e lifetimes (`10-traits-generics-lifetimes.md`)

Comportamento compartilhado, bounds, tipos genericos, derive, Display e lifetimes.

- `trait-define-impl` - Definir trait e implementar (Intermediario)
- `trait-default-method` - Metodo default em trait (Avancado)
- `trait-as-parameter` - Receber parametro com trait bound (Intermediario)
- `impl-trait-return` - Retornar impl Trait (Avancado)
- `generic-function` - Funcao generica com trait bound (Avancado)
- `generic-struct` - Struct generica (Intermediario)
- `where-clause` - Usar where clause (Avancado)
- `derive-debug-clone` - Derivar traits comuns (Intermediario)
- `display-impl` - Implementar Display (Avancado)
- `struct-lifetime-ref` - Struct com referencia e lifetime (Avancado)

### Arrays e slices (`11-arrays-slices.md`)

Arrays de tamanho fixo, slices, string slices e APIs que recebem fatias.

- `array-fixed-size` - Criar array de tamanho fixo (Iniciante)
- `array-repeated-values` - Criar array com valores repetidos (Iniciante)
- `slice-from-array` - Criar slice a partir de array (Iniciante)
- `fn-recebe-slice` - Funcao que recebe &[i32] (Intermediario)
- `mutable-slice` - Alterar parte de array via &mut [i32] (Intermediario)
- `vec-as-slice` - Passar Vec para funcao que recebe slice (Intermediario)
- `string-slice-range` - Criar &str com range (Intermediario)
- `split-at` - Dividir slice com split_at (Intermediario)

### Iteradores (`12-iterators.md`)

iter, iter_mut, into_iter, adaptadores e consumidores idiomaticos.

- `iter-sum` - Somar valores com iter().sum() (Iniciante)
- `iter-enumerate` - Usar enumerate para indice e valor (Iniciante)
- `iter-filter-map` - Filtrar e transformar com filter_map (Intermediario)
- `iter-find` - Encontrar primeiro item (Iniciante)
- `iter-any-all` - Usar any e all (Intermediario)
- `iter-zip` - Combinar duas colecoes com zip (Intermediario)
- `iter-fold` - Acumular resultado com fold (Intermediario)
- `iter-partition` - Separar itens com partition (Avancado)
- `iter-copied-cloned` - Diferenca entre copied e cloned (Intermediario)
- `into-iter-consume` - Consumir colecao com into_iter (Intermediario)

### Pattern matching (`13-pattern-matching.md`)

Match com ranges, guards, if let, while let, matches! e desestruturacao.

- `match-number-range` - Usar match com ranges (Iniciante)
- `match-guard` - Usar guarda com if no match (Intermediario)
- `if-let-option` - Tratar Some com if let (Iniciante)
- `while-let-pop` - Consumir Vec com while let (Intermediario)
- `matches-macro` - Usar matches! (Iniciante)
- `destructure-struct` - Desestruturar struct em match (Intermediario)
- `destructure-enum-data` - Extrair dados de enum (Intermediario)
- `ignore-values` - Ignorar valores com _ e .. (Intermediario)

### Testes (`14-tests.md`)

#[test], asserts, should_panic, testes com Result e mod tests.

- `test-basic-assert` - Criar teste com assert! (Iniciante)
- `test-assert-eq` - Testar retorno com assert_eq! (Iniciante)
- `test-should-panic` - Testar panic esperado (Intermediario)
- `test-result-return` - Teste retornando Result (Intermediario)
- `cfg-test-module` - Criar mod tests com #[cfg(test)] (Intermediario)
- `test-helper-function` - Usar funcao auxiliar em teste (Intermediario)

### Modulos, use e pub (`15-modules-crates.md`)

Organizacao em modulos, caminhos, visibilidade e APIs publicas em um arquivo.

- `mod-inline` - Criar modulo inline (Iniciante)
- `mod-file` - Simular modulo de arquivo separado (Iniciante)
- `use-path` - Importar item com use (Iniciante)
- `pub-function` - Expor funcao publica (Iniciante)
- `pub-struct-fields` - Struct publica com campos privados (Intermediario)
- `impl-public-api` - Criar API publica simples (Intermediario)

### Tratamento de erros (`16-error-handling.md`)

Result, Option para Result, transformacoes, propagacao e erros customizados.

- `result-match` - Tratar Result com match (Iniciante)
- `result-unwrap-or-else` - Usar unwrap_or_else (Iniciante)
- `result-map` - Transformar Ok com map (Intermediario)
- `result-map-err` - Transformar erro com map_err (Intermediario)
- `option-ok-or` - Converter Option em Result com ok_or (Intermediario)
- `parse-int-result` - Converter string para numero tratando erro (Intermediario)
- `custom-error-enum` - Criar enum de erro simples (Avancado)
- `from-error` - Implementar From para converter erro (Avancado)

### CLI e entrada de usuario (`17-cli-input.md`)

stdin, read_line, parse de entrada e argumentos de linha de comando.

- `stdin-read-line` - Ler linha do terminal (Intermediario)
- `stdin-trim-parse` - Ler numero e converter (Intermediario)
- `args-collect` - Ler argumentos com std::env::args (Iniciante)
- `args-match-command` - Interpretar comando simples (Intermediario)

### Smart pointers (`18-smart-pointers.md`)

Box, Rc, RefCell e combinacoes para cenarios avancados de posse.

- `box-value` - Guardar valor no heap com Box (Intermediario)
- `box-recursive-enum` - Enum recursivo com Box (Avancado)
- `rc-shared-ownership` - Compartilhar dado com Rc (Avancado)
- `refcell-interior-mutability` - Mutabilidade interior com RefCell (Avancado)
- `rc-refcell` - Combinar Rc<RefCell<T>> (Avancado)

