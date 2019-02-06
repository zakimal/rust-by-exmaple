use std::fmt::{self, Display, Formatter};
use std::mem;
use List::*;

// 'Structure'という構造体のための'fmt::Debug'をderive（自動生成）する
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        (write!(f, "["))?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                (write!(f, ", "))?;
            }
            (write!(f, "{}", v))?;
        }
        write!(f, "]")
    }
}

struct City {
    name: &'static str, // プログラムが生きているうちは死なない変数
    lat: f32,
    lon: f32,
}
impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let r = self.red;
        let g = self.green;
        let b = self.blue;
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", r, g, b, r, g, b)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer) // ';'なしでこの関数の返り値
}

#[derive(Debug)]
struct Matrix(f64, f64, f64, f64);
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

struct Nil; // ユニット
struct Pair(i32, f64);
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Skinny => println!("Is skinny!"),
        Person::Fat => println!("Is Fat!"),
        Person::Height(i) => println!("Has a height of {}", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        }
    }
}

enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}
enum Number { // iota
    Zero,
    One,
    Two,
}
enum EnumColor {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
//enum List {
//    Cons(u32, Box<List>), // Lispのリストみたいなもの
//    Nil,
//}
//impl List {
//    fn new() -> List {
//        Nil
//    }
//    fn prepend(self, elem: u32) -> List {
//        Cons(elem, Box::new(self))
//    }
//    fn len(&self) -> u32 {
//        match *self {
//            Cons(_, ref tail) => 1 + tail.len(),
//            Nil => 0,
//        }
//    }
//    fn stringify(&self) -> String {
//        match *self {
//            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
//            Nil => format!("Nil"),
//        }
//    }
// }

static LANGUAGE: &'static str = "rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    // 1. hello world
    println!("Hello, world!");
    // 1.1 コメント
    // '//'で行末までコメント
    // '/*    */'でブロック内コメント
    // '///'でドキュメント用コメント
    // '//!'でソースコードのドキュメント用コメント

    //------------------------------------------------------------------------------------

    // 1.2 フォーマットしてプリント
    // std::fmtで定義されるいくつかのマクロによって印字することができる。
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("{} of {:b} people know binary, the other half don't", 1, 2); // ':'に続けてフォーマット指定
    println!("{number:>width$}", number = 1, width = 6); // 指定した幅で右寄せ
    println!("{number:>0width$}", number = 1, width = 6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    struct Structure(i32);
    // println!("This struct '{}' won't print...", Structure3);
    println!("Pi is roughly {:.*}", 3, 22f64 / 7f64);
    println!("this is for {:?}", "debug"); // {:?}でデバッグ形式で出力
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // println!("Now {:?} will print!", Structure(3));
    // println!("Now {:?} will print!", Deep(Structure(7)));
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("\tDisplay: {}", minmax);
    println!("\tDebug: {:?}", minmax);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    let point = Point2 { x: 2.2, y: 7.2 };
    println!("Compare points");
    println!("\tDisplay: {}", point);
    println!("\tDebug: {:?}", point);
    let c = Complex { re: 3.3, im: 7.2 };
    println!("Compare complex");
    println!("\tDisplay: {}", c);
    println!("\tDebug: {:?}", c);
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    // 文字列がどのようにフォーマットされるかはフォーマット文字列によって決まる。
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{:?}", *color);
        println!("{}", *color)
    }

    //------------------------------------------------------------------------------------

    // 2. 基本データ型
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32; // suffixで型指定
    let default_float = 3.0;
    let default_integer = 7;
    let mut mutable = 12;
    // mutable = true; // mutableでも型は不変
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    println!("0011 AND 0101 is {:04b}", 0b011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b1100u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    println!("One million is written as {}", 1_000_000u32); // 可読性のための'_'
                                                            // タプル：異なる型の値の集合
    let long_touple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long tuple first value: {}", long_touple.0);
    println!("long tuple second value: {}", long_touple.1);
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));
    println!("one element tuple needs comma: {:?}", (5u32,));
    println!("\tcompare to an just integer: {:?}", 5u32);
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
    // 配列とスライス
    // 配列は単一の型のオブジェクトの集合で、サイズ込みで型。[T; size]
    // スライスは配列の一部への参照。ポインタとサイズで構成される。&[T]
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // 0で初期化された長さ500の配列
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs)); // 配列は長さ固定なのでヒープではなくスタックに保持
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
    // println!("{}", xs[5]) // index out of range

    //------------------------------------------------------------------------------------

    // 3. カスタム型
    // 'struct': 構造体を定義する
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };
    let _nil = Nil;
    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    // 'enum': 列挙型を定義する
    let person = Person::Height(18);
    let danny = Person::Weight(10);
    let dave = Person::Info {name: "Dave".to_owned(), height: 72};
    let john = Person::Fat;
    let larry = Person::Skinny;
    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
    // 'use'することで絶対名でなくても使用可能。
    use Status::{Poor, Rich};
    use Work::*;
    let status = Poor;
    let work = Civilian;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor has no money..."),
    }
    match work {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", EnumColor::Red as i32);
    println!("violets are #{:06x}", EnumColor::Blue as i32);
//    let mut list = List::new();
//    list = list.prepend(1);
//    list = list.prepend(2);
//    list = list.prepend(3);
//    println!("linked list has length: {}", list.len());
//    println!("{}", list.stringify())
    // 'const'/'static': 定数を定義する
    // 'const':不変な値
    // 'static':プログラムが動作している間は不変な値
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});
    // THRESHOLD = 5 // 変更できない

}

// '{}'というマーカーを使うためにはその型専用の'fmt::Display'が実装されている必要がある
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

