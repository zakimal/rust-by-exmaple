use std::fmt::Debug;

// 普通のトレイトとその実装
trait HasArea {
    fn area(&self) -> f64;
}
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64, height: f64
}
#[allow(dead_code)]
struct Triangle  {
    length: f64, height: f64
}

// ジェネリック境界：任意の型ではなくて、型の中でもDebugトレイトを実装している型に限定した関数
fn print_debug<T: Debug>(t: T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle{length: 3.0, height: 4.0};
    let triangle = Triangle{length: 3.0, height: 4.0};
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle))
}