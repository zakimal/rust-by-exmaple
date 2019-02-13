use std::num::ParseIntError;
use std::result;

// エイリアス
type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    number_str.parse::<i32>().map(|n| 2*n)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e)
    }
}

fn main() {
    print(double_number("10"));
    print(double_number("t"));
}