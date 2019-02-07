struct Point {
    x: f64,
    y: f64
}
impl Point { // 構造体'Point'のメソッド群を定義する
    // 引数に&selfを取らないのはスタティックメソッドで、インスタンスから出なくても呼び出せる。コンストラクタとして使用することが多い
    fn origin() -> Point { Point {x: 0.0, y: 0.0} }
    fn new(x: f64, y: f64) -> Point { Point {x, y} }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}
impl Rectangle {
    // インスタンスメソッド
    fn area(&self) -> f64 {
        let Point{x: x1, y: y1} = self.p1; // デコンストラクタ
        let Point{x: x2, y: y2} = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }
    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) { // 呼び出し元インスタンスがmutableであることが必要
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);
impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self; // デコンストラクト
        println!("Destroying Pair({} {})", first, second);
        // ここを抜けた段階でfirstとsecondが解放される
    }
}

fn main() {
    // メソッドとはオブジェクトに付属した関数のこと
    // 'impl'で定義する
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 2.0),
    };
    // rectangle.translate(1.0, 1.0);
    square.translate(1.0, 2.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}