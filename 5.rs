#![allow(overflowing_literals)] // オーバーフローを起こすようなキャスティングによる警告を無視する

fn main() {
    // rustではプリミティブ型における強制的な型キャストを暗黙に行うことはない。
    let decimal = 65.4321_f32;
    // let integer: u8 = decimal; // だめ
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    // 何らかの値を符号無しの型Tに変換すると値がその変換先の型Tの許す範囲に収まるまでstd::T::MAX+1が加算もしくは減算される
    println!("1000 as a u16 is {}", 1000 as u16);
    println!("1000 as a u8 is {}", 1000 as u8); // 超えちゃってるから256が何回か引かれることになる
    println!("-1 as a u8 is {}", (-1i8) as u8);
    println!("1000 mod 256 is {}", 1000 % 256);
    // 符号なし型を符号あり型にキャストすると、対応する符号無しの型にキャストするか、2の補数を取る
    println!("128 as a i8 is {}", 128 as i8); // -128
    println!("1000 as a i8 is {}", 1000 as i8);
    println!("232 as a i8 is {}", 232 as i8);
}