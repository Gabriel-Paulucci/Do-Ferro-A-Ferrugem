const NUMERO_CONST: i32 = 10;

fn main() {
    let numero = 5;
    // numero = 10; isso gera um erro de compilação

    println!("{}", numero);

    let mut numero_mut = 5;
    numero_mut = 10; // isso funciona normalmente

    println!("{}", numero_mut);

    // NUMERO_CONST = 5; // isso gera um erro de compilação

    println!("{}", NUMERO_CONST);
}
