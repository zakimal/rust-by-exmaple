fn main() {
    // rustの型推論機構は賢い。
    // 初期化の際に何が与えられるかだけではなく、
    // その先その値がどのように使われるかも見ている
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}