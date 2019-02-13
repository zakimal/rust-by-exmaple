// 変数には、自分の保持するメモリ領域という資源を不要になったら解放する責任があるので、
// 逆に資源は一度に一つの所有者を持つと言える。
// 変数をアサインする際や、関数に引数を値渡しする際には資源の所有権がmoveする。

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn main() {
    let x = 5u32;
    let y = x;
    println!("x is {}, y is {}", x, y);

    let a = Box::new(5i32); // aはヒープ上の32ビット整数5へのポインタ
    println!("a contains {}", a);

    let b = a; // move
    // aもbもヒープ上の同じ位置を指しているが、所有権はbにあるのでaはアクセスできない
    // println!("a contains {}", a);
    destroy_box(b); // ここで値渡しでbを関数destroy_boxに引き渡してしまうので変数bは所有権を失う
    // println!("b contains {}", b);
}