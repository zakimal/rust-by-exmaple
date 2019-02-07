fn main() {
    println!("Tell me type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1 ... 12 => println!("I'm a child of age {:?}", n), // '@'をつけて'n'にバインディング
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn age() -> u32 {
    15
}