fn main() {
    // 関数を引数に取ることもできる
    let closure = || println!("I'm a closure!");
    call_function(closure);
    call_function(print)
}

fn call_function<F: Fn()>(f: F) { // トレイト'Fn'を実装している型の集合を'F'という名前にして、型'F'のものを引数に取る関数を定義
    f()
}

fn print() {
    println!("I'm a function!")
}
