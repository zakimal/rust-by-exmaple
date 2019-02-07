macro_rules! test {
    // 「式; and 式」に対してprintlnしてstringifyして結果を計算して表示するというマクロ
    ($left:expr; and $right:expr) => (println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right));
    // 「式; or 式」に対してprintlnしてstringifyして結果を計算して表示するというマクロ
    ($left:expr; or $right:expr) => (println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right));
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}