fn main() {
    println!("Hello world!");
    let x = 7;
    let print = || println!("{}", x); // トレイト'F'を実装しているクロージャを'print'という名前で束縛
    apply(print); // これを実行する
}

fn apply<F>(f: F) where F: Fn() {
    f(); // 引数'f'はトレイト'F'を実装していて、それを呼び出す
}