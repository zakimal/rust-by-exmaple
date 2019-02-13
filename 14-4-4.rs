// i32への参照をメンバーにもつBorrowed型を定義。
// Borrowed型の構造体に格納される参照は、Borrowed構造体自体よりも長生きでないとならない。
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32)
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed{
        x: &x, y: &y,
    };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);
    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}