fn main() {
    // パターンマッチ：Cで言うところの'switch'みたいなもの。
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("This is ONE"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime (< 13)"),
        13...19 => println!("A teen"),
        _ => println!("Ain't special"), // '_'でなんでも受ける
    }
    // パターンマッチは式文でもある
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}