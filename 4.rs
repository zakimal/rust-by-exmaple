fn main() {
    // rustは静的な型付け言語なので安全。つまりぬるぽしない。
    // 静的な型付けだけど、関数内に限って賢い型推論機構が働くので型を明示するので忙しすぎるようなことはない
    let an_integer = 1u32;
    let a_boolean = true;
    let unit =  ();
    let copied_integer = an_integer;
    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);
    // 使用されない変数があるとコンパイラが怒る。ただし'_'をつければ黙る。
    let _unused_value = 3u32;
    let _noisy_unused_value = 2u32;
}
