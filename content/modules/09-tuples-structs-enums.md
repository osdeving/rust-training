id: tuples-structs-enums
title: Tuplas, structs e enums
description: Modelagem de dados com tuplas, structs, metodos, funcoes associadas e enums.

## tuple-destructure
module_id: tuples-structs-enums
title: Desestruturar tupla
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado ponto, desestruture a tupla em x e y.
```
```scaffold
let ponto = (10, 20);
```
```summary
Tuplas agrupam valores por posicao e podem ser desestruturadas com padrao.
```
```solution
let ponto = (10, 20);
let (x, y) = ponto;
```
### Hints
- O padrao do lado esquerdo tem a mesma forma da tupla.
### Concepts
- Desestruturacao cria nomes para partes de um valor composto.
### Pitfalls
- Tuplas usam posicao, nao nomes de campos.
### Docs
- [The Rust Book: Tuple type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
### Rules
- required_pattern|destructure|let\s*\(\s*x\s*,\s*y\s*\)\s*=|Desestruturou a tupla.|Esperava desestruturar em x e y.

## tuple-access
module_id: tuples-structs-enums
title: Acessar campos de tupla por indice
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Dado usuario, crie nome usando o campo 0 e idade usando o campo 1.
```
```scaffold
let usuario = ("Ana", 32);
```
```summary
Campos de tupla podem ser acessados com .0, .1, .2 e assim por diante.
```
```solution
let usuario = ("Ana", 32);
let nome = usuario.0;
let idade = usuario.1;
```
### Hints
- Use a notacao ponto mais indice.
### Concepts
- A ordem dos campos faz parte do significado da tupla.
### Pitfalls
- .0 nao e chamada de metodo.
### Docs
- [The Rust Book: Tuple indexes](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
### Rules
- required_pattern|campo-zero|usuario\s*\.\s*0|Acessou primeiro campo.|Esperava acessar o campo 0.
- required_pattern|campo-um|usuario\s*\.\s*1|Acessou segundo campo.|Esperava acessar o campo 1.

## struct-named-fields
module_id: tuples-structs-enums
title: Criar struct com campos nomeados
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Declare uma struct Pessoa com nome: String e idade: u8, depois crie uma instancia.
```
```summary
Structs com campos nomeados deixam o dominio mais claro que tuplas longas.
```
```solution
struct Pessoa {
    nome: String,
    idade: u8,
}

let pessoa = Pessoa {
    nome: String::from("Ana"),
    idade: 32,
};
```
### Hints
- Defina os campos dentro da declaracao da struct.
- A instancia usa NomeDaStruct { campo: valor }.
### Concepts
- Campos nomeados documentam a intencao dos dados.
### Pitfalls
- Todos os campos sem valor default precisam ser preenchidos.
### Docs
- [The Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
### Rules
- required_pattern|struct|struct\s+Pessoa\s*\{|Declarou struct Pessoa.|Esperava declarar uma struct.
- required_pattern|nome|nome\s*:\s*String|Declarou campo nome.|Esperava campo nome.
- required_pattern|idade|idade\s*:\s*u8|Declarou campo idade.|Esperava campo idade.
- required_pattern|instancia|Pessoa\s*\{|Criou instancia da struct.|Esperava instanciar Pessoa.

## struct-field-update
module_id: tuples-structs-enums
title: Atualizar struct mutavel
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Declare uma struct Contador { valor: i32 }, crie contador mutavel e incremente valor em 1.
```
```summary
Campos de struct podem ser alterados quando a instancia e mutavel.
```
```solution
struct Contador {
    valor: i32,
}

let mut contador = Contador { valor: 0 };
contador.valor += 1;
```
### Hints
- A mutabilidade fica na variavel que guarda a struct.
### Concepts
- let mut permite alterar campos da instancia.
### Pitfalls
- Marcar o campo como mut dentro da struct nao existe em Rust.
### Docs
- [The Rust Book: Mutable structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#changing-a-value-in-a-struct)
### Rules
- required_pattern|struct|struct\s+Contador\s*\{|Declarou Contador.|Esperava declarar Contador.
- required_ast|mutavel|HasLetMut|Criou instancia mutavel.|Esperava let mut.
- required_pattern|campo|contador\s*\.\s*valor|Acessou campo valor.|Esperava usar o campo valor.

## tuple-struct
module_id: tuples-structs-enums
title: Criar tuple struct
difficulty: Beginner
compile_mode: SnippetAsMain
```prompt
Declare uma tuple struct Metros(f64), crie distancia e leia o campo interno.
```
```summary
Tuple structs dao nome a um tipo sem exigir nomes para cada campo.
```
```solution
struct Metros(f64);

let distancia = Metros(12.5);
let valor = distancia.0;
```
### Hints
- A declaracao parece uma funcao, mas termina com ponto e virgula.
### Concepts
- Newtypes ajudam a evitar misturar unidades diferentes.
### Pitfalls
- O campo interno continua acessado por indice.
### Docs
- [The Rust Book: Tuple structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
### Rules
- required_pattern|tuple-struct|struct\s+Metros\s*\(\s*f64\s*\)\s*;|Declarou tuple struct.|Esperava Metros(f64).
- required_pattern|instancia|Metros\s*\(|Criou instancia.|Esperava instanciar Metros.
- required_pattern|campo|distancia\s*\.\s*0|Leu campo interno.|Esperava acessar campo 0.

## impl-method
module_id: tuples-structs-enums
title: Implementar metodo em struct
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare Retangulo { largura: u32, altura: u32 } e implemente area(&self) -> u32.
```
```summary
Blocos impl adicionam metodos associados a um tipo.
```
```solution
struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

let retangulo = Retangulo { largura: 4, altura: 3 };
let area = retangulo.area();
```
### Hints
- Metodos que leem a instancia recebem &self.
### Concepts
- self representa a instancia atual dentro do metodo.
### Pitfalls
- Esquecer &self transforma area em funcao associada sem receptor.
### Docs
- [The Rust Book: Method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
### Rules
- required_pattern|struct|struct\s+Retangulo\s*\{|Declarou Retangulo.|Esperava struct Retangulo.
- required_pattern|impl|impl\s+Retangulo\s*\{|Criou bloco impl.|Esperava impl Retangulo.
- required_pattern|area|fn\s+area\s*\(\s*&\s*self\s*\)\s*->\s*u32|Definiu metodo area.|Esperava metodo area com &self.
- required_pattern|campos|self\s*\.\s*largura\s*\*\s*self\s*\.\s*altura|Calculou area com campos.|Esperava multiplicar largura e altura.

## associated-function
module_id: tuples-structs-enums
title: Criar funcao associada new
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare Usuario { nome: String } e implemente Usuario::new(nome: String) -> Usuario.
```
```summary
Funcoes associadas ficam no impl, mas nao recebem self. new e uma convencao para construtores.
```
```solution
struct Usuario {
    nome: String,
}

impl Usuario {
    fn new(nome: String) -> Usuario {
        Usuario { nome }
    }
}

let usuario = Usuario::new(String::from("Ana"));
```
### Hints
- new nao recebe self porque cria a instancia.
### Concepts
- Field init shorthand permite Usuario { nome }.
### Pitfalls
- Usuario::new chama funcao associada; usuario.new chamaria metodo.
### Docs
- [The Rust Book: Associated functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)
### Rules
- required_pattern|impl|impl\s+Usuario\s*\{|Criou impl Usuario.|Esperava bloco impl.
- required_pattern|new|fn\s+new\s*\(\s*nome\s*:\s*String\s*\)\s*->\s*Usuario|Criou funcao associada new.|Esperava Usuario::new.
- required_pattern|chamada|Usuario::new\s*\(|Chamou funcao associada.|Esperava chamar Usuario::new.

## enum-basic-match
module_id: tuples-structs-enums
title: Criar enum e tratar com match
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare enum Status com Ativo e Inativo. Depois use match para criar mensagem.
```
```summary
Enums representam alternativas fechadas; match obriga tratar os casos.
```
```solution
enum Status {
    Ativo,
    Inativo,
}

let status = Status::Ativo;
let mensagem = match status {
    Status::Ativo => "ok",
    Status::Inativo => "parado",
};
```
### Hints
- Cada variante e acessada como Status::Variante.
### Concepts
- match precisa cobrir todas as variantes.
### Pitfalls
- Nomes de variantes normalmente usam PascalCase.
### Docs
- [The Rust Book: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
### Rules
- required_pattern|enum|enum\s+Status\s*\{|Declarou enum Status.|Esperava declarar enum.
- required_pattern|ativo|Ativo|Incluiu variante Ativo.|Esperava variante Ativo.
- required_pattern|inativo|Inativo|Incluiu variante Inativo.|Esperava variante Inativo.
- required_ast|match|HasMatch|Tratou com match.|Esperava usar match.

## enum-data-variant
module_id: tuples-structs-enums
title: Enum com dados associados
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare enum Evento com Click { x: i32, y: i32 } e Tecla(char). Crie um Evento::Tecla.
```
```summary
Variantes de enum podem carregar dados nomeados, posicionais ou nenhum dado.
```
```solution
enum Evento {
    Click { x: i32, y: i32 },
    Tecla(char),
}

let evento = Evento::Tecla('q');
```
### Hints
- Uma variante pode parecer uma struct e outra uma tuple struct.
### Concepts
- Enums modelam tipos soma: um valor e uma entre varias formas.
### Pitfalls
- Os dados so existem quando aquela variante foi escolhida.
### Docs
- [The Rust Book: Enum values](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
### Rules
- required_pattern|enum|enum\s+Evento\s*\{|Declarou enum Evento.|Esperava enum Evento.
- required_pattern|click|Click\s*\{\s*x\s*:\s*i32\s*,\s*y\s*:\s*i32\s*\}|Criou variante com campos.|Esperava Click com x e y.
- required_pattern|tecla|Tecla\s*\(\s*char\s*\)|Criou variante com char.|Esperava Tecla(char).
- required_pattern|instancia|Evento::Tecla\s*\(|Criou variante Tecla.|Esperava criar Evento::Tecla.

## struct-destructure
module_id: tuples-structs-enums
title: Desestruturar struct
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Declare Ponto { x: i32, y: i32 }, crie um ponto e desestruture em x e y.
```
```summary
Structs tambem podem ser desestruturadas por nome de campo.
```
```solution
struct Ponto {
    x: i32,
    y: i32,
}

let ponto = Ponto { x: 3, y: 4 };
let Ponto { x, y } = ponto;
```
### Hints
- O padrao usa o nome do tipo e os campos.
### Concepts
- Desestruturar struct evita acessar campo por campo depois.
### Pitfalls
- Em structs, a ordem dos campos nao importa; os nomes importam.
### Docs
- [The Rust Book: Patterns with structs](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#destructuring-structs)
### Rules
- required_pattern|struct|struct\s+Ponto\s*\{|Declarou Ponto.|Esperava struct Ponto.
- required_pattern|destructure|let\s+Ponto\s*\{\s*x\s*,\s*y\s*\}\s*=|Desestruturou por campos.|Esperava desestruturar Ponto.
