// Iteratorトレイトは、要素の集合のような型に対してイテレーターを生成するトレイトを提供する。
// 実装は自前でやるかデフォルトで提供されるものを使う。

struct Fibonacci {
    current: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}

fn fibonacci_generator() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1
    }
}

fn main() {
    let mut sequence = 0..3;
    println!("Four consecutive next calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using 'for'");
    for i in 0..3 {
        println!("> {}", i)
    }

    println!("the first four terms of the Fibonacci sequence are: ");
    for i in fibonacci_generator().take(4) {
        println!("> {}", i);
    }

    println!("the next four terms of the Fibonacci sequence are:");
    for i in fibonacci_generator().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}