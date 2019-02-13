use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// Refは'aというライフタイムを持つジェネリック型Tに対する参照を持ち
// Tの値に対する参照は必ず'aよりも長生きでなくてはならない。
// さらにRefのライフタイムは'aを超えてはならない。

fn print<T>(t: T) where T: Debug {
    println!("print: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("print_red: t is {:?}", t);
}
// print_ref関数は、Debugトレイトを実装していてかつライフタイムが'最低でも'aより長いジェネリック型Tに対するライフタイム'aの関数

fn main() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}