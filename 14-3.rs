// 一時的にそのデータにアクセスすることを借用と表現する。借用では値そのものTではなくて、その参照&Tを渡す。
// Rustコンパイラは借用チェッカーを用いて、リファレンスが常に有効なオブジェクトへの参照であるかを確認する。
// つまり、誰かが借りている間はそのオブジェクトを破壊することはできないことが保証される。

fn eat_box(boxed_int: Box<i32>) {
    println!("Destroying box that contains {}", boxed_int);
}

fn borrow_box(borrowed_int: &i32) {
    println!("This int is {}", borrowed_int);
}

fn main() {
    let boxed_int = Box::new(5);
    borrow_box(&boxed_int); // &はborrowを表す
    borrow_box(&boxed_int);
    {
        let _ref_to_int: &i32 = &boxed_int; // ここでboxed_intに対する借用を取っている
        // eat_box(boxed_int); // 最深のスコープ内で_ret_to_intがboxed_intを借用しているので食べることは許されない
    }
    eat_box(boxed_int);
}