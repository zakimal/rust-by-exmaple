static NUM: i32 = 18; // staticなライフタイム（プログラムが動作している間は常に確保されている）を持つ

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM // ここでstaticが圧縮されている
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);
}