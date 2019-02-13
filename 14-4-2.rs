// 引数として'aのライフタイムを持つ参照を一つとる関数のライフタイムは最低でも'aより長くなくてはならない
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_muti: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn main() {
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z = pass_x(&x, &y);
    print_one(z);
    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}