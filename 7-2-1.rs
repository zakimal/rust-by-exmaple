#![allow(unreachable_code)]

fn main() {
    'outer: loop { // ラベル付け
        println!("Entered the outer loop!");
        'inner: loop {
            println!("Entered the inner loop!!");
            // break; // inner loopだけ抜ける
            break 'outer;
        }
        println!("This point will never be reached.")
    }
    println!("Exit the outer loop");
}