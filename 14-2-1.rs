// データの操作可能性は所有権のムーブ時に変更できる

fn main() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
    // *immutable_box = 4;
    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box contains {}", mutable_box);
}