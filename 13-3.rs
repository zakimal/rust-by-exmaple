struct Empty;
struct Null;

// ジェネリクストレイト：任意の型Tに対するインタフェースみたいなもの
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {} // ここで引数として型TとUは受けているので所有権がムーブする
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null) // emptyとnullがメモリから同時に解放される
}