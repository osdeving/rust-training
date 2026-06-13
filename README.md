# Rust Training

135 exercicios curtos para memorizar as construcoes de Rust.

Rust Training nao tenta ser livro, curso em video ou explicacao conceitual longa.
Ele e um app de pratica deliberada para quem ja sabe programar e quer ganhar
velocidade escrevendo Rust: ownership, borrowing, slices, iteradores, `match`,
`Result`, `HashMap`, structs, traits, testes, modulos e outras construcoes que
aparecem no codigo real.

A ideia e direta: abrir o app, resolver exercicios pequenos, receber validacao,
ver uma solucao guiada quando travar e repetir ate a sintaxe ficar natural.

## Para quem e

- Pessoas que ja programam em outra linguagem e querem praticar Rust sem perder tempo.
- Devs que ja leram o basico, mas ainda travam para lembrar "como escreve isso em Rust".
- Quem quer fazer um sprint de fim de semana para sair codando coisas pequenas em Rust.

## Para quem nao e

- Nao e um curso teorico completo sobre Rust.
- Nao substitui a documentacao oficial, o Rust Book ou pratica em projetos reais.
- Nao executa o codigo do usuario; ele valida estrutura, regex, AST e compilacao.

## O que tem no MVP

- App desktop com Tauri.
- CLI instalavel via Cargo.
- 19 modulos e 135 exercicios.
- Exercicios carregados por Markdown.
- Catalogo automatico em `content/CATALOG.md`.
- Validacao por AST com `syn`, regex e `rustc`.
- Solucao guiada com mini-doc, armadilhas e links oficiais.
- Progresso local, dashboard, navegacao anterior/proximo e reload do catalogo.
- Editor com syntax highlighting na UI.
- Conteudo embutido no binario para o CLI funcionar fora do repositorio.

## Instalacao rapida

### Desktop, recomendado para usuarios finais

Baixe o instalador na pagina de GitHub Releases do projeto.

- Windows: use o instalador `.msi` ou `.exe`.
- macOS: use o `.dmg` ou `.app.tar.gz`.
- Linux: use `.AppImage`, `.deb` ou `.rpm`, conforme o artefato gerado.

O app desktop abre sem uma toolchain Rust instalada. Porem, a validacao de
compilacao usa `rustc`; para o botao de verificar compilar respostas, instale
Rust pelo `rustup`. O app checa a toolchain ao abrir, tenta localizar `rustc`
no `PATH` e em caminhos comuns do `rustup`, e mostra um aviso quando precisar
instalar ou reabrir o aplicativo.

### CLI via Cargo

Depois que o crate estiver publicado no crates.io:

```bash
cargo install rust-training
rust-training list
rust-training show vec-inferido-push
rust-training hint vec-inferido-push 2
rust-training check vec-inferido-push resposta.rs
```

Tambem da para validar usando stdin:

```bash
rust-training check vec-inferido-push < resposta.rs
```

Enquanto o projeto ainda nao estiver publicado:

```bash
cargo install --path .
rust-training list
```

## Instalacao por sistema

### Windows

Para usar o desktop:

1. Baixe o instalador em GitHub Releases.
2. Instale e abra `Rust Training`.
3. Para validar compilacao, instale Rust:

```powershell
winget install --id Rustlang.Rustup
rustup default stable-msvc
```

Para desenvolver ou gerar instalador localmente, instale tambem Microsoft C++
Build Tools e WebView2 quando o sistema ainda nao tiver. O Tauri usa WebView2
para renderizar a UI no Windows.

### macOS

Para usar o desktop, baixe o `.dmg` em GitHub Releases.

Para validar compilacao ou desenvolver:

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

### Linux

Para usar o desktop, baixe o artefato Linux em GitHub Releases. Em algumas
distros, apps Tauri dependem das bibliotecas WebKitGTK do sistema.

Para validar compilacao ou desenvolver em Debian/Ubuntu:

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
```

## Rodando do codigo fonte

```bash
git clone https://github.com/<seu-usuario>/rust-training.git
cd rust-training
cargo test
cargo run -- list
```

O app desktop nao tem etapa de build frontend nem dependencia de npm. A UI
estatica fica em `ui/`.

```bash
cargo install tauri-cli --version "^2"
cargo tauri dev
```

Gerar instaladores locais:

```bash
cargo tauri build
```

No Windows deste ambiente, a CLI foi instalada em `D:\rust-training\.cargo-home`
para evitar o drive C sem espaco. Nesse caso:

```powershell
$env:PATH="D:\rust-training\.cargo-home\bin;C:\Users\osdev\.cargo\bin;$env:PATH"
$env:CARGO_HOME="D:\rust-training\.cargo-home"
$env:TEMP="D:\rust-training\.tmp"
$env:TMP="D:\rust-training\.tmp"
cargo tauri dev
```

## Publicacao

### GitHub

O caminho recomendado para o MVP desktop e GitHub Releases:

1. Suba o codigo para um repositorio GitHub.
2. Garanta que a branch principal seja `main`.
3. Rode a CI em pull requests e pushes na `main`.
4. Crie uma tag:

```bash
git tag v0.1.0
git push origin main --tags
```

O workflow `.github/workflows/release.yml` compila Windows, macOS e Linux com
`cargo tauri build`. Em tags `v*`, ele anexa os bundles diretamente na GitHub
Release; em execucoes manuais, ele deixa os bundles como artifacts da action.

### crates.io

Da para publicar no Cargo, mas isso deve ser entendido como distribuicao do CLI.
O app desktop continua sendo melhor distribuido por GitHub Releases, winget,
Homebrew, Scoop ou lojas depois.

Antes do primeiro publish:

```bash
cargo package --list
cargo publish --dry-run
```

Depois, com conta e token do crates.io:

```bash
cargo login
cargo publish
```

Notas praticas:

- O nome `rust-training` precisa estar livre no crates.io.
- Publicacao no crates.io e permanente para cada versao; nao da para sobrescrever
  uma versao ja publicada.
- O CLI inclui os Markdown embutidos no binario. Se `content/modules/` existir no
  diretorio atual, essa pasta ganha prioridade para facilitar desenvolvimento.

## Checklist de release

Antes de publicar um MVP:

- Atualizar `version` em `Cargo.toml`, `src-tauri/Cargo.toml` e `src-tauri/tauri.conf.json`.
- Rodar `cargo fmt`.
- Rodar `cargo test --workspace`.
- Rodar `cargo check -p rust-training-tauri`.
- Rodar `cargo publish --dry-run`.
- Criar tag `vX.Y.Z`.
- Conferir os artifacts do workflow de release.
- Revisar a GitHub Release gerada com texto curto, screenshots e links de instalacao.

## Catalogo dinamico de exercicios

Os modulos e exercicios sao carregados em runtime a partir de arquivos Markdown:

```text
content/modules/
```

Durante desenvolvimento, o binario nao precisa ser recompilado para adicionar,
remover ou editar exercicios. Ao subir o app novamente, ele rele essa pasta.

Com o app desktop aberto, use o botao `Recarregar` na barra superior para reler
os Markdown sem reiniciar. Isso atualiza modulos, exercicios e textos da solucao
guiada. Se o exercicio atual ainda existir, ele continua aberto; se tiver sido
removido, o app volta para a pagina inicial.

Tambem e possivel apontar outra pasta:

```powershell
$env:RUST_TRAINING_CONTENT_DIR="D:\meus-exercicios"
cargo tauri dev
```

Cada arquivo `.md` representa um modulo. A ordem default e alfabetica pelo nome
dos arquivos, entao use prefixos como `01-vectors.md`, `02-strings.md`, etc.

Quando nenhuma pasta externa e encontrada, o CLI usa os Markdown embutidos no
binario. Isso permite `cargo install rust-training` sem copiar a pasta
`content/` junto.

### Sumario automatico

Ao carregar os modulos, o app tambem sincroniza um snapshot enxuto em:

```text
content/CATALOG.md
```

Esse arquivo e gerado automaticamente a partir de `content/modules/*.md`. Ele
mostra a lista de modulos, o arquivo de origem, a quantidade de exercicios, a
distribuicao por dificuldade e um indice com todos os exercicios.

O fluxo e:

1. O app le todos os Markdown em `content/modules/`.
2. Os modulos continuam sendo a fonte da verdade.
3. O app renderiza o sumario esperado.
4. Se `content/CATALOG.md` nao existe ou esta diferente, ele reescreve o arquivo.
5. Se estiver igual, nada muda.

Na pratica, voce nao edita `content/CATALOG.md` manualmente. Edite os modulos,
rode `cargo run -- list`, `cargo test` ou clique em `Recarregar` no app, e o
sumario sera corrigido. Se o app estiver empacotado em um local sem permissao de
escrita, a falha de atualizar o sumario nao impede o catalogo de carregar.

### Como o parser le os Markdown

O parser e propositalmente simples. Ele nao interpreta Markdown completo; ele
reconhece uma estrutura combinada de metadados, titulos, blocos cercados por
crase e listas.

No topo do arquivo ficam os metadados do modulo:

```markdown
id: strings
title: Strings
description: Conversoes, consultas e processamento de texto.
```

Depois disso, cada exercicio comeca com um titulo `##`, e o texto depois de `##`
vira o `id` do exercicio:

```markdown
## string-trim-parse
```

Dentro de um exercicio, o parser entende:

- Metadados no formato `chave: valor`.
- Blocos de codigo nomeados: `prompt`, `summary`, `solution` e `scaffold`.
- Secoes de lista `### Hints`, `### Concepts`, `### Pitfalls`, `### Docs` e `### Rules`.
- Itens de lista iniciados com `- ` dentro dessas secoes.

Tudo que nao estiver nessa estrutura e ignorado. Isso ajuda a manter o formato
previsivel, mas significa que comentarios livres fora dos blocos e listas nao
entram no app.

### Campos de modulo

Todo arquivo de modulo precisa ter:

- `id`: identificador estavel do modulo. Use kebab-case ou snake_case, sem espacos.
- `title`: nome mostrado na UI.
- `description`: resumo curto mostrado na lista e na pagina inicial.

O nome do arquivo controla a ordem natural. Prefira prefixos numericos:

```text
01-vectors.md
02-strings.md
09-types-traits.md
```

### Campos de exercicio

Todo exercicio precisa ter:

- `## exercicio-id`: identificador unico do exercicio no catalogo inteiro.
- `module_id`: deve bater com o `id` do modulo.
- `title`: titulo mostrado na UI.
- `prompt`: bloco obrigatorio com o enunciado.
- `summary`: bloco obrigatorio usado como introducao da solucao guiada.
- `solution`: bloco obrigatorio com a solucao sugerida. Ela tambem e usada pelos
  testes do catalogo para garantir que o exercicio e valido.

Campos opcionais:

- `difficulty`: `Beginner`, `Intermediate` ou `Advanced`. Se omitido, assume `Beginner`.
- `compile_mode`: `SnippetAsMain`, `FullProgram` ou `Off`. Se omitido, assume `SnippetAsMain`.
- `scaffold`: codigo inicial que aparece no editor. Use quando quiser dar
  boilerplate sem entregar o ponto principal do exercicio.
- `Hints`: dicas progressivas mostradas pelo botao de dica.
- `Concepts`: conceitos exibidos na mini-doc da solucao guiada.
- `Pitfalls`: armadilhas comuns exibidas na mini-doc.
- `Docs`: links oficiais no formato `[Titulo](https://url)`.
- `Rules`: rubrica de validacao.

### Modos de compilacao

Use `compile_mode` para dizer como a resposta sera checada pelo `rustc`:

- `SnippetAsMain`: o app coloca a resposta dentro de uma funcao `main`. Use para
  exercicios curtos com `let`, chamadas de metodo, structs pequenas etc.
- `FullProgram`: a resposta deve ser um arquivo Rust completo. Use quando o
  exercicio pede `fn main() -> Result<...>`, leitura de arquivos ou imports no
  nivel do arquivo.
- `Off`: pula compilacao. Use raramente, apenas para exercicios puramente
  estruturais ou quando a sintaxe aceita ainda nao compila isolada.

Mesmo quando uma regra passa por regex ou AST, a compilacao ainda e importante:
ela evita que o aluno apenas escreva palavras esperadas em codigo invalido.

### Como adicionar um exercicio a um modulo existente

Para adicionar um item ao modulo de strings, por exemplo:

1. Abra `content/modules/02-strings.md`.
2. Va ate o fim do arquivo.
3. Crie um novo bloco `## novo-id-unico`.
4. Preencha os metadados obrigatorios.
5. Escreva `prompt`, `summary` e `solution`.
6. Adicione `Hints`, `Concepts`, `Pitfalls`, `Docs` e `Rules`.
7. Rode `cargo test` para validar se a solucao sugerida passa nas regras.
8. Com o app aberto, clique em `Recarregar`.

Exemplo minimo:

````markdown
## string-replace-bug
module_id: strings
title: Trocar trecho de texto
difficulty: Beginner
compile_mode: SnippetAsMain

```prompt
Dado texto, crie corrigido trocando "bug" por "fix".
```

```summary
replace cria uma nova String com as ocorrencias substituidas.
```

```solution
let texto = "bug pequeno";
let corrigido = texto.replace("bug", "fix");
```

```scaffold
let texto = "bug pequeno";
```

### Hints
- Procure um metodo de string que substitui trechos.

### Concepts
- replace retorna uma nova String.

### Pitfalls
- replace troca todas as ocorrencias encontradas.

### Docs
- [str::replace](https://doc.rust-lang.org/std/primitive.str.html#method.replace)

### Rules
- required_ast|replace|HasMethodCall|replace|Usou substituicao de texto.|Esperava substituir parte do texto.
````

### Como pensar um exercicio

Comece pelo objetivo de aprendizagem, nao pela regra.

Por exemplo, se voce quer validar se o dev aprendeu a usar `HashMap`, pode criar
um exercicio que pede para contar ou registrar valores por chave. O enunciado
poderia pedir: "Dado um mapa de pontos, insira a pontuacao de Ana e depois leia
esse valor". Para validar, voce pode exigir que ele use a estrutura `HashMap`, o
metodo de insercao e o metodo de consulta:

```text
- required_pattern|hashmap|HashMap|Usou HashMap.|Esperava usar um mapa.
- required_ast|insert|HasMethodCall|insert|Inseriu um valor.|Esperava inserir no mapa.
- required_ast|get|HasMethodCall|get|Consultou um valor.|Esperava consultar no mapa.
```

Se o objetivo e treinar `match`, nao valide uma string exata da solucao. Peca
algo como "trate `Option<i32>` retornando 0 quando nao houver valor" e valide a
estrutura:

```text
- required_ast|match|HasMatch|Tratou os casos com match.|Esperava usar match.
- required_pattern|some|Some\s*\(|Tratou Some.|Esperava tratar valor presente.
- required_pattern|none|None|Tratou None.|Esperava tratar ausencia.
```

Se o objetivo e ensinar uma API especifica de string, como `split_whitespace`,
voce pode criar um exercicio que pede para separar palavras ignorando espacos
extras. A validacao deve exigir a API principal e o fechamento do iterador:

```text
- required_ast|split-whitespace|HasMethodCall|split_whitespace|Separou por espacos.|Esperava separar palavras.
- required_ast|collect|HasMethodCall|collect|Coletou o iterador.|Esperava coletar o resultado.
```

A regra pratica e: valide o comportamento estrutural que interessa para o
aprendizado, mas evite prender o aluno a nomes de variaveis ou formatacao exata
quando isso nao for o foco.

## Regras de validacao

Formato geral:

```text
- tipo|id|argumentos...|mensagem_de_sucesso|mensagem_de_falha
```

Cada regra vira uma linha no resultado da verificacao. A `mensagem_de_sucesso`
aparece quando a regra passa; a `mensagem_de_falha` aparece quando ela falha.
Escreva mensagens pedagogicas e curtas.

Tipos:

- `required_ast`: exige uma estrutura de AST. Melhor para metodo, macro, `if`,
  `match`, loop, funcao e checks ja suportados.
- `forbidden_ast`: reprova se a estrutura aparecer. Bom para proibir `unwrap`,
  `vec!`, concatenacao com `+`, etc.
- `required_pattern`: exige regex no texto. Use quando o AST ainda nao tem um
  check especifico, por exemplo `struct Pessoa`, `impl Trait for Tipo`,
  lifetime ou formato de assinatura.
- `forbidden_pattern`: reprova regex no texto. Use para bloquear atalhos textuais.
- `min_pattern_count`: exige que uma regex apareca pelo menos N vezes.

Exemplos:

```text
- required_ast|if|HasIf|Usou if.|Esperava if/else.
- required_ast|push|HasMethodCall|push|Usou push().|Esperava .push(...).
- required_ast|capacidade|HasCallPathWithIntArg|Vec::with_capacity|3|Usou capacidade 3.|Esperava Vec::with_capacity(3).
- min_pattern_count|dois-for|\bfor\b|2|Usou dois loops for.|Esperava dois loops for.
- forbidden_ast|sem-unwrap|HasMethodCall|unwrap|Nao usou unwrap().|Evite unwrap() neste exercicio.
```

### Quando usar AST e quando usar regex

Prefira `required_ast` sempre que existir um `AstCheck` adequado. Ele ignora
comentarios e e menos fragil que procurar texto. Por exemplo,
`HasMethodCall|push` detecta uma chamada real de metodo `push`, enquanto uma
regex por `push` tambem poderia bater em comentario, string ou nome de variavel.

Use `required_pattern` quando voce precisa validar uma forma que o AST collector
ainda nao conhece. Exemplos atuais: `struct Pessoa`, `trait Descrever`,
`impl Descrever for Produto`, uma assinatura generica especifica ou lifetime em
campo.

Evite regex muito rigida. Em vez de exigir uma solucao inteira em uma linha,
valide partes relevantes. Isso deixa o aluno livre para escrever codigo
equivalente.

Como `|` e separador das regras, evite `|` literal dentro das regex. Se precisar
de alternancia, prefira duas regras ou uma regex sem esse caractere.

`AstCheck` aceitos:

```text
HasLetMut
HasLetType|Vec
HasCallPath|Vec::new
HasCallPathWithIntArg|Vec::with_capacity|3
HasMethodCall|push
HasMacroCall|format
HasForLoop
HasForLoopByReference
HasWhileLoop
HasLoop
HasIf
HasMatch
HasFunction
HasFunctionParamCount|2
HasFunctionReturnType
HasArrayToVec
HasBinaryAdd
MinMethodCallCount|push|2
```

### Checklist para revisar um Markdown novo

Antes de considerar um exercicio pronto:

- O `id` do exercicio e unico no catalogo inteiro.
- O `module_id` bate com o `id` do modulo.
- `prompt`, `summary` e `solution` existem.
- A solucao sugerida passa nas regras.
- As regras validam o conceito central, nao apenas uma variavel com nome especifico.
- As dicas ajudam sem entregar a solucao inteira.
- A mini-doc tem pelo menos um link oficial quando o assunto tem documentacao clara.
- `compile_mode` esta adequado ao tipo de resposta.

Comandos uteis:

```bash
cargo run -- list
cargo run -- show string-replace-bug
cargo test
```

## Arquitetura

- `src/domain.rs`: tipos centrais de modulo, exercicio, regra e resultado.
- `src/content.rs`: loader/parser dos Markdown em `content/modules/`.
- `src/embedded_content.rs`: fallback com exercicios embutidos para o binario instalado.
- `src/ast.rs`: coleta fatos estruturais do codigo Rust com `syn`.
- `src/analyzer.rs`: motor de validacao das rubricas sobre AST, regex e compilacao.
- `src/compiler.rs`: checagem de compilacao do snippet sem executar o programa.
- `src/catalog.rs`: carrega o catalogo dinamico.
- `src/main.rs`: CLI fina em cima da biblioteca.
- `src-tauri/`: shell desktop Tauri e comandos Rust expostos para a UI.
- `ui/`: interface estatica carregada pelo Tauri.

Para adicionar outro modulo, crie um arquivo `.md` em `content/modules/`. Nao e
necessario recompilar o binario para alterar o conteudo durante desenvolvimento.

## Limitacoes

A analise AST evita boa parte dos falsos positivos de regex, mas ainda nao
entende semantica profunda de Rust. Ela identifica estrutura do codigo, nao
prova comportamento.

A checagem completa ainda depende de `rustc` instalado na maquina onde o app
roda. Sem `rustc`, a compilacao falha, embora o app abra e mostre o catalogo
normalmente. Se o Rust tiver sido instalado enquanto o app estava aberto, clique
em `Rechecar` ou feche e abra novamente.

## Referencias de empacotamento

- [Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Tauri: Distribute](https://v2.tauri.app/distribute/)
- [Tauri: Prerequisites](https://v2.tauri.app/start/prerequisites/)
