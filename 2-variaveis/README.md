# Tipos de variaveis

Boa noite a todos, venho aqui hoje apresentar os tipos de variáveis existentes e como usar cada uma da melhor maneira possivel, e quais são os tipos básicos e suas variações.

Basicamente existem 3 tipos de variáveis, mas inicialmente irei explicar o uso de comente duas, pois uma dela depende de alguns conceitos mais avançados.

## let e let mut

O `let` é o tipo de variável mais comumente usados, pois ele é usado para instanciar qualquer tipo de dado, não é obrigatório definir o tipo da variável no `let`.

```rust
let numero = 5;
```

Variáveis instanciadas usando somente o `let` são imutáveis, ou seja, não podem ter o seu valor alterado durante o codigo.

```rust
let numero = 5;
numero = 10; // isso gera um erro de compilação
```

Então para tornar uma variável mutável, se usa junto a palavra-chave `mut`.

```rust
let mut numero = 5;
numero = 10; // funcionara sem problemas
```

Realmente a unica diferença dessa variação é que uma pode ser alterada e a outra não, mas sempre prefira manter so mente o `let` sempre que possivel, mas não é errado o uso do `let mut`, somente fica mais visível que algo não foi feito para ser alterado.

## const

O tipo de variavel `const` serve para definir constantes no seu programa, nada alem disso.

```rust
const NUMERO_CONST: i32 = 5;
```

Nesse caso é obrigatório definir a tipagem da variável, e por convenção o nome das variáveis deve ser todo maiúsculo.
Esse tipo de variável não pode ser alterado durante o programa, sendo usada somente para definir constantes.

```rust
const NUMERO_CONST: i32 = 10;

fn main() {
    NUMERO_CONST = 5; // isso gera um erro de compilação
}
```
