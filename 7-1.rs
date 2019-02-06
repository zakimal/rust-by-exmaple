fn main() {
    // if-elseは式文なのでどの分岐に進んでも返り値が同じ型である必要がある。
    let n = 5;
    if n < 0 {
        print!("{} is negative!", n);
    } else if n > 0 {
        print!("{} is positive!", n);
    } else {
        print!("{} is zero!", n);
    }
    let big_n = {
        if n < 10 && n > -10 {
            println!(", and is small number, increase ten-fold");
            10 * n // 10倍した値が返る
        } else {
            println!(", and is a big number, reduce by two");
            n / 2
        }
    };
    println!("{} -> {}", n, big_n);
}