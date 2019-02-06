fn main() {
    // サフィックスで型を指定できる
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1; // defaultはi32
    let f = 1.0; // defaultはf64
    println!("sizeof 'x' in bytes is {} = {} bits", std::mem::size_of_val(&x), std::mem::size_of_val(&x) * 8);
    println!("sizeof 'y' in bytes is {} = {} bits", std::mem::size_of_val(&y), std::mem::size_of_val(&y) * 8);
    println!("sizeof 'z' in bytes is {} = {} bits", std::mem::size_of_val(&z), std::mem::size_of_val(&z) * 8);
    println!("sizeof 'i' in bytes is {} = {} bits", std::mem::size_of_val(&i), std::mem::size_of_val(&i) * 8);
    println!("sizeof 'f' in bytes is {} = {} bits", std::mem::size_of_val(&f), std::mem::size_of_val(&f) * 8);
}