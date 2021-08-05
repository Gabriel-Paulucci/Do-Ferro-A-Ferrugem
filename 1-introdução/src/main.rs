fn main() {
    let result: u128 = fibonacci(20);
    println!("{}", result);
}

fn fibonacci(n: u128) -> u128 {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
