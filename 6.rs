fn main() {
    // rustのプログラムはStatementの連続で構成される
    // コードブロックも文の一種で、それをそのまま右辺値として扱うことも可能。
    // その場合、コードブロック内の最後の式文が左辺値に代入される。
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x * x * x;
        x_cube + x_squared + x // これが右辺値として代入される
    };
    let z = {
        2 * x; // この場合は何も返らない
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}