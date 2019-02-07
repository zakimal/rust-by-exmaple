fn main() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
         // パターンマッチ内で条件文をつけてフィルタリングするのがguard
        (x, y) if x == y => println!("There are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}