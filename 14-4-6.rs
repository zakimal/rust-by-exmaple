fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// <'a: 'b, 'b>はライフタイム'aは最低でも'bと同じ長さ
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;
    {
        let second = 3;
        println!("the product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}