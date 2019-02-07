struct Val (f64,); // 具象構造体
struct GenVal<T>(T,); // ジェネリクス構造体

// 具象構造体へのメソッドを定義
impl Val {
    fn value(&self) -> &f64 {
        &self.0
    }
}

// ジェネリクス構造体へのメソッドを定義。T = f64とすればValと同じ
impl <T> GenVal<T> {
    fn value(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = Val(3.0);
    let y = GenVal(3i32);
    println!("{:?} {:?}", x.value(), y.value());
}