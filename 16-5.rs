use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
    match number_str.parse::<i32>() {
        Ok(n) => Ok(2*n),
        Err(e) => Err(e),
    }
}

fn double_number_map(number_str: &str) -> Result<i32, ParseIntError> {
    number_str.parse::<i32>().map(|n| 2 * n)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 以前と同様、問題なく想定通りの値を表示する。
    let twenty = double_number("10");
    print(twenty);

    // 以前の`panic`の内容よりも遥かに良い。
    let tt = double_number_map("t");
    print(tt);
}