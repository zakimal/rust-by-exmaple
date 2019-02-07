fn main() {
    // 構造体Fooの定義
    struct Foo {
        x: (u32, u32),
        y: u32
    }
    let foo = Foo {x: (1, 2), y: 3};
    // Fooの型に沿ってfooから流し込む感じ
    let Foo {x: (a, b), y} = foo; // ここでdestruct
    println!("a = {}, b = {}, y = {}", a, b, y);
    let Foo {y: i, x : j} = foo;
    println!("i = {:?}, j = {:?}", i, j);
    let Foo {y, ..} = foo; // 一部無視
    println!("y = {}", y);
}