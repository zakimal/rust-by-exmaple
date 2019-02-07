// 関数の名前identをとって自分の名前を印字するマクロ
macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("you called {:?}()", stringify!($func_name))
        }
    )
}

 // マクロから関数を作る
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => (
        println!("{:?} = {:?}", stringify!($expression), $expression)
    )
}

fn main() {
    foo();
    bar();
    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}