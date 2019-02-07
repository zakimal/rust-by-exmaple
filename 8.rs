fn main() {
    // 関数は'fn'キーワードで定義できる
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () { // 値を返さない関数
    if is_divisible_by(n, 15) {
        println!("FizzBuzz");
    } else if is_divisible_by(n, 3) {
        println!("Fizz");
    } else if is_divisible_by(n, 5) {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n :u32) {
    for n in 1..n+1 {
        fizzbuzz(n);
    }
}