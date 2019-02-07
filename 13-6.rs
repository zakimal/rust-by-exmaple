use std::fmt::Debug;

// PrintInOptionというトレイト
trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}