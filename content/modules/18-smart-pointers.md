id: smart-pointers
title: Smart pointers
description: Box, Rc, RefCell e combinacoes para cenarios avancados de posse.

## box-value
module_id: smart-pointers
title: Guardar valor no heap com Box
difficulty: Intermediate
compile_mode: SnippetAsMain
```prompt
Crie um Box com Box::new(42) e depois leia o valor usando desreferencia.
```
```summary
Box<T> guarda um valor no heap e deixa um ponteiro possuidor na stack.
```
```solution
let numero = Box::new(42);
let valor = *numero;
```
### Hints
- Use dereference com *.
### Concepts
- Box tem posse unica do valor.
### Pitfalls
- Box nao cria compartilhamento automatico.
### Docs
- [The Rust Book: Box<T>](https://doc.rust-lang.org/book/ch15-01-box.html)
### Rules
- required_ast|box-new|HasCallPath|Box::new|Criou Box.|Esperava Box::new.
- required_ast|deref|HasLetInitializerWithDeref|Desreferenciou Box.|Esperava guardar o valor lido com *.

## box-recursive-enum
module_id: smart-pointers
title: Enum recursivo com Box
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Declare enum Lista { Cons(i32, Box<Lista>), Nil } e crie uma lista com dois itens.
```
```summary
Box permite que enums recursivos tenham tamanho conhecido.
```
```solution
enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}

let lista = Lista::Cons(1, Box::new(Lista::Cons(2, Box::new(Lista::Nil))));
```
### Hints
- A recursao precisa ficar atras de Box.
### Concepts
- Sem Box, o enum teria tamanho infinito.
### Pitfalls
- Box resolve tamanho, nao compartilhamento.
### Docs
- [The Rust Book: Recursive types with Box](https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes)
### Rules
- required_pattern|enum|enum\s+Lista\s*\{|Declarou enum recursivo.|Esperava enum Lista.
- required_pattern|box-field|Box\s*<\s*Lista\s*>|Usou Box no campo recursivo.|Esperava Box<Lista>.
- required_ast|box-new|HasCallPath|Box::new|Criou Box.|Esperava Box::new.

## rc-shared-ownership
module_id: smart-pointers
title: Compartilhar dado com Rc
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Use std::rc::Rc para compartilhar uma String entre a e b com Rc::clone.
```
```summary
Rc<T> permite multiplos donos em codigo single-thread.
```
```solution
use std::rc::Rc;

let texto = Rc::new(String::from("dados"));
let a = Rc::clone(&texto);
let b = Rc::clone(&texto);
```
### Hints
- Clone de Rc incrementa contador, nao copia a String.
### Concepts
- Rc usa contagem de referencias.
### Pitfalls
- Rc nao e thread-safe; para threads existe Arc.
### Docs
- [The Rust Book: Rc<T>](https://doc.rust-lang.org/book/ch15-04-rc.html)
### Rules
- required_pattern|use-rc|use\s+std::rc::Rc|Importou Rc.|Esperava use Rc.
- required_ast|rc-new|HasCallPath|Rc::new|Criou Rc.|Esperava Rc::new.
- required_pattern|rc-clone|Rc::clone\s*\(\s*&\s*texto\s*\)|Clonou Rc.|Esperava Rc::clone.

## refcell-interior-mutability
module_id: smart-pointers
title: Mutabilidade interior com RefCell
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Use RefCell::new(0), depois borrow_mut para alterar o valor interno.
```
```summary
RefCell<T> checa regras de borrowing em runtime e permite mutabilidade interior.
```
```solution
use std::cell::RefCell;

let contador = RefCell::new(0);
*contador.borrow_mut() += 1;
```
### Hints
- borrow_mut retorna um guard mutavel.
### Concepts
- RefCell e util quando o compilador nao consegue provar o padrao seguro estaticamente.
### Pitfalls
- Violacao das regras de borrowing gera panic em runtime.
### Docs
- [The Rust Book: RefCell<T>](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
### Rules
- required_pattern|use-refcell|use\s+std::cell::RefCell|Importou RefCell.|Esperava use RefCell.
- required_ast|refcell-new|HasCallPath|RefCell::new|Criou RefCell.|Esperava RefCell::new.
- required_ast|borrow-mut|HasMethodCall|borrow_mut|Pegou emprestimo mutavel.|Esperava borrow_mut.

## rc-refcell
module_id: smart-pointers
title: Combinar Rc<RefCell<T>>
difficulty: Advanced
compile_mode: SnippetAsMain
```prompt
Crie contador = Rc::new(RefCell::new(0)), clone para outro e incremente via borrow_mut.
```
```summary
Rc<RefCell<T>> combina compartilhamento single-thread com mutabilidade interior.
```
```solution
use std::cell::RefCell;
use std::rc::Rc;

let contador = Rc::new(RefCell::new(0));
let outro = Rc::clone(&contador);
*outro.borrow_mut() += 1;
```
### Hints
- Rc compartilha; RefCell permite mutar o interior.
### Concepts
- Esse padrao aparece em grafos, arvores com pais e estados compartilhados.
### Pitfalls
- Pode criar ciclos de Rc; Weak ajuda nesses casos.
### Docs
- [The Rust Book: Rc and RefCell](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects)
### Rules
- required_ast|rc-new|HasCallPath|Rc::new|Criou Rc.|Esperava Rc::new.
- required_ast|refcell-new|HasCallPath|RefCell::new|Criou RefCell.|Esperava RefCell::new.
- required_pattern|rc-clone|Rc::clone\s*\(\s*&\s*contador\s*\)|Clonou Rc.|Esperava Rc::clone.
- required_ast|borrow-mut|HasMethodCall|borrow_mut|Mutou interior.|Esperava borrow_mut.
