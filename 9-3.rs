macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => ( // 「$(...),+」と書くことで「...」にマッチする表現に対してマクロを適用できる
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}